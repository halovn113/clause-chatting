mod commands;
mod llm;
mod memory;
mod tools;

use commands::AppState;
use llm::llama::LlamaCppProvider;
use memory::MemoryManager;
use parking_lot::RwLock;
use std::sync::Arc;
use tools::builtin;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let llm = Arc::new(LlamaCppProvider::new().expect("Failed to init llama.cpp"));

    let mut tool_registry = tools::ToolRegistry::new();
    builtin::register_builtin_tools(&mut tool_registry);

    let memory_dir = dirs_next::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("butler")
        .join("memory");

    let memory = MemoryManager::new(memory_dir)
        .map(|m| Arc::new(RwLock::new(Some(m))))
        .unwrap_or_else(|_| Arc::new(RwLock::new(None)));

    let state = AppState {
        llm,
        tools: Arc::new(RwLock::new(tool_registry)),
        memory,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::llm_load_model,
            commands::llm_generate,
            commands::llm_health,
            commands::tool_list,
            commands::tool_execute,
            commands::memory_read,
            commands::memory_write,
            commands::memory_list,
            commands::memory_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
