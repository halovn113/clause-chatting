use anyhow::Result;
use tokio::sync::mpsc;

use crate::llm::types::{ChatMessage, LlmConfig, LlmResponse, StreamChunk};

pub mod llama;
pub mod types;

pub trait ILlmProvider: Send + Sync {
    fn generate(
        &self,
        messages: &[ChatMessage],
        config: &LlmConfig,
    ) -> Result<LlmResponse>;

    fn stream(
        &self,
        messages: &[ChatMessage],
        config: &LlmConfig,
    ) -> Result<mpsc::Receiver<StreamChunk>>;

    fn health(&self) -> Result<bool>;

    fn embedding(&self, _text: &str) -> Result<Vec<f32>> {
        Ok(vec![])
    }
}
