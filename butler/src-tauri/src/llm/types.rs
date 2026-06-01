use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: &str) -> Self {
        Self { role: "system".into(), content: content.into() }
    }

    pub fn user(content: &str) -> Self {
        Self { role: "user".into(), content: content.into() }
    }

    pub fn assistant(content: &str) -> Self {
        Self { role: "assistant".into(), content: content.into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub model_path: String,

    pub context_size: u32,

    #[serde(default = "default_threads")]
    pub threads: u32,

    #[serde(default = "default_batch_size")]
    pub batch_size: u32,

    #[serde(default = "default_gpu_layers")]
    pub gpu_layers: u32,

    #[serde(default = "default_temp")]
    pub temperature: f32,

    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

fn default_threads() -> u32 { 4 }
fn default_batch_size() -> u32 { 512 }
fn default_gpu_layers() -> u32 { 0 }
fn default_temp() -> f32 { 0.7 }
fn default_max_tokens() -> u32 { 4096 }

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            model_path: String::new(),
            context_size: 8192,
            threads: default_threads(),
            batch_size: 512,
            gpu_layers: 0,
            temperature: 0.7,
            max_tokens: 4096,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub content: String,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub token: String,
    pub done: bool,
}
