use std::sync::Arc;

use parking_lot::RwLock;
use tauri::State;

use crate::llm::llama::LlamaCppProvider;
use crate::llm::types::{ChatMessage, LlmConfig, LlmResponse};
use crate::llm::ILlmProvider;
use crate::memory::MemoryManager;
use crate::tools::{ToolCall, ToolRegistry};

pub struct AppState {
    pub llm: Arc<LlamaCppProvider>,
    pub tools: Arc<RwLock<ToolRegistry>>,
    pub memory: Arc<RwLock<Option<MemoryManager>>>,
}

// ── LLM Commands ────────────────────────────────────────────

#[tauri::command]
pub async fn llm_load_model(
    state: State<'_, AppState>,
    config: LlmConfig,
) -> Result<(), String> {
    state.inner().llm.load_model(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn llm_generate(
    state: State<'_, AppState>,
    messages: Vec<ChatMessage>,
    config: LlmConfig,
) -> Result<LlmResponse, String> {
    state.inner().llm.generate(&messages, &config).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn llm_health(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    state.inner().llm.health().map_err(|e| e.to_string())
}

// ── Tool Commands ───────────────────────────────────────────

#[tauri::command]
pub async fn tool_list(
    state: State<'_, AppState>,
) -> Result<Vec<crate::tools::ToolInfo>, String> {
    Ok(state.inner().tools.read().list())
}

#[tauri::command]
pub async fn tool_execute(
    state: State<'_, AppState>,
    call: ToolCall,
) -> Result<crate::tools::ToolResult, String> {
    Ok(state.inner().tools.read().execute(&call))
}

// ── Memory Commands ─────────────────────────────────────────

#[tauri::command]
pub async fn memory_read(
    state: State<'_, AppState>,
    category: String,
) -> Result<String, String> {
    let mem = state.inner().memory.read();
    match mem.as_ref() {
        Some(m) => m.read_memory(&category).map_err(|e| e.to_string()),
        None => Err("Memory not initialized".into()),
    }
}

#[tauri::command]
pub async fn memory_write(
    state: State<'_, AppState>,
    category: String,
    key: String,
    content: String,
) -> Result<(), String> {
    let mem = state.inner().memory.read();
    match mem.as_ref() {
        Some(m) => m.write_memory(&category, &key, &content).map_err(|e| e.to_string()),
        None => Err("Memory not initialized".into()),
    }
}

#[tauri::command]
pub async fn memory_list(
    state: State<'_, AppState>,
    category: String,
) -> Result<Vec<String>, String> {
    let mem = state.inner().memory.read();
    match mem.as_ref() {
        Some(m) => m.list_entries(&category).map_err(|e| e.to_string()),
        None => Err("Memory not initialized".into()),
    }
}

#[tauri::command]
pub async fn memory_delete(
    state: State<'_, AppState>,
    category: String,
    key: String,
) -> Result<(), String> {
    let mem = state.inner().memory.read();
    match mem.as_ref() {
        Some(m) => m.delete_memory(&category, &key).map_err(|e| e.to_string()),
        None => Err("Memory not initialized".into()),
    }
}
