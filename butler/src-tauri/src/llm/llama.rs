use std::num::NonZero;
use std::sync::Arc;

use anyhow::{Context, Result};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::model::AddBos;
#[allow(deprecated)]
use llama_cpp_2::model::Special;
use llama_cpp_2::token::data_array::LlamaTokenDataArray;
use parking_lot::RwLock;
use tokio::sync::mpsc;

use super::ILlmProvider;
use crate::llm::types::{ChatMessage, LlmConfig, LlmResponse, StreamChunk};

pub struct LlamaCppProvider {
    model: Arc<RwLock<Option<LlamaModel>>>,
    context: Arc<RwLock<Option<LlamaContext<'static>>>>,
    backend: LlamaBackend,
}

unsafe impl Send for LlamaCppProvider {}
unsafe impl Sync for LlamaCppProvider {}

impl LlamaCppProvider {
    pub fn new() -> Result<Self> {
        let backend = LlamaBackend::init()
            .context("Failed to initialize llama.cpp backend")?;

        Ok(Self {
            model: Arc::new(RwLock::new(None)),
            context: Arc::new(RwLock::new(None)),
            backend,
        })
    }

    pub fn load_model(&self, config: &LlmConfig) -> Result<()> {
        let model_params = LlamaModelParams::default()
            .with_n_gpu_layers(config.gpu_layers);

        let model = LlamaModel::load_from_file(
            &self.backend,
            &config.model_path,
            &model_params,
        )
        .context("Failed to load GGUF model")?;

        let ctx_size = NonZero::new(config.context_size)
            .context("Context size must be > 0")?;

        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(Some(ctx_size))
            .with_n_batch(config.batch_size)
            .with_n_ubatch(config.batch_size)
            .with_n_threads(config.threads as i32)
            .with_n_threads_batch(config.threads as i32);

        let context = model
            .new_context(&self.backend, ctx_params)
            .context("Failed to create inference context")?;

        // SAFETY: model is moved into Arc and lives for 'static, so context
        // (which borrows from model) is also valid for 'static
        let context: LlamaContext<'static> = unsafe {
            std::mem::transmute::<LlamaContext<'_>, LlamaContext<'static>>(context)
        };

        *self.model.write() = Some(model);
        *self.context.write() = Some(context);
        Ok(())
    }
}

impl ILlmProvider for LlamaCppProvider {
    fn generate(
        &self,
        messages: &[ChatMessage],
        config: &LlmConfig,
    ) -> Result<LlmResponse> {
        let model_guard = self.model.read();
        let model = model_guard
            .as_ref()
            .context("Model not loaded. Call load_model first.")?;

        let mut ctx_guard = self.context.write();
        let context = ctx_guard
            .as_mut()
            .context("Context not created")?;

        let prompt = format_messages(messages);
        let tokens = model
            .str_to_token(&prompt, AddBos::Always)
            .context("Failed to tokenize prompt")?;

        let n_tokens = tokens.len();
        if n_tokens > config.context_size as usize {
            anyhow::bail!(
                "Prompt too long: {} tokens, max context: {}",
                n_tokens,
                config.context_size
            );
        }

        let mut batch = LlamaBatch::new(n_tokens, 1);

        for (i, &token) in tokens.iter().enumerate() {
            let is_last = i == n_tokens - 1;
            batch.add(token, i as i32, &[0], is_last)
                .context("Failed to add token to batch")?;
        }

        context.decode(&mut batch)
            .context("Failed to decode prompt")?;

        let mut output = String::new();
        let eos_token = model.token_eos();
        let max_chars = (config.max_tokens * 4) as usize;
        let mut n_cur = batch.n_tokens();

        while n_cur <= config.context_size as i32 {
            let candidates = context.candidates_ith(batch.n_tokens() - 1);
            let mut candidates_p = LlamaTokenDataArray::from_iter(candidates, false);

            let new_token_id = candidates_p.sample_token(42);

            if new_token_id == eos_token || output.len() >= max_chars {
                break;
            }

            if let Ok(s) = model.token_to_str(new_token_id, Special::Tokenize) {
                output.push_str(&s);
            }

            batch.clear();
            batch.add(new_token_id, n_cur, &[0], true)
                .context("Failed to add next token")?;

            context.decode(&mut batch)
                .context("Failed to decode next token")?;

            n_cur += 1;
        }

        Ok(LlmResponse {
            content: output,
            finish_reason: "stop".into(),
        })
    }

    fn stream(
        &self,
        _messages: &[ChatMessage],
        _config: &LlmConfig,
    ) -> Result<mpsc::Receiver<StreamChunk>> {
        let (tx, rx) = mpsc::channel(256);
        let _ = tx; // Placeholder — will implement properly later
        Ok(rx)
    }

    fn health(&self) -> Result<bool> {
        Ok(self.model.read().is_some())
    }
}

fn format_messages(messages: &[ChatMessage]) -> String {
    let mut prompt = String::new();
    for msg in messages {
        match msg.role.as_str() {
            "system" => prompt.push_str(&format!("<|im_start|>system\n{}<|im_end|>\n", msg.content)),
            "user" => prompt.push_str(&format!("<|im_start|>user\n{}<|im_end|>\n", msg.content)),
            "assistant" => prompt.push_str(&format!("<|im_start|>assistant\n{}<|im_end|>\n", msg.content)),
            _ => prompt.push_str(&format!("<|im_start|>{}\n{}<|im_end|>\n", msg.role, msg.content)),
        }
    }
    prompt.push_str("<|im_start|>assistant\n");
    prompt
}
