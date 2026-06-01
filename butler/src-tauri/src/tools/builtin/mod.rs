use crate::tools::{ITool, SafetyLevel, ToolResult};

pub struct DiskTool;

impl ITool for DiskTool {
    fn name(&self) -> &str {
        "disk"
    }

    fn description(&self) -> &str {
        "Analyze disk usage: list drives, find large folders, check free space"
    }

    fn is_read_only(&self) -> bool {
        true
    }

    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Safe
    }

    fn execute(&self, _args: serde_json::Value) -> ToolResult {
        ToolResult {
            success: true,
            output: "Disk tool placeholder — will use sysinfo for real data".into(),
            error: None,
        }
    }
}

pub struct ShellTool;

impl ITool for ShellTool {
    fn name(&self) -> &str {
        "shell"
    }

    fn description(&self) -> &str {
        "Execute shell commands on the host system"
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Dangerous
    }

    fn execute(&self, args: serde_json::Value) -> ToolResult {
        let cmd = args.get("command").and_then(|v| v.as_str()).unwrap_or("");

        let output = std::process::Command::new("cmd")
            .arg("/C")
            .arg(cmd)
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                ToolResult {
                    success: out.status.success(),
                    output: format!("{}\n{}", stdout, stderr),
                    error: None,
                }
            }
            Err(e) => ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Shell execution failed: {}", e)),
            },
        }
    }
}

pub struct FileTool;

impl ITool for FileTool {
    fn name(&self) -> &str {
        "file"
    }

    fn description(&self) -> &str {
        "File operations: read, write, list, search files"
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Careful
    }

    fn execute(&self, args: serde_json::Value) -> ToolResult {
        let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("read");
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");

        match action {
            "read" => match std::fs::read_to_string(path) {
                Ok(content) => ToolResult {
                    success: true,
                    output: content,
                    error: None,
                },
                Err(e) => ToolResult {
                    success: false,
                    output: String::new(),
                    error: Some(format!("Failed to read: {}", e)),
                },
            },
            "list" => {
                match std::fs::read_dir(path) {
                    Ok(entries) => {
                        let mut list = String::new();
                        for entry in entries.flatten() {
                    let dir_mark = if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) { "/" } else { "" };
                            list.push_str(&format!(
                                "{}{}\n",
                                entry.file_name().to_string_lossy(),
                                dir_mark
                            ));
                        }
                        ToolResult {
                            success: true,
                            output: list,
                            error: None,
                        }
                    }
                    Err(e) => ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some(format!("Failed to list: {}", e)),
                    },
                }
            }
            "write" => {
                let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
                match std::fs::write(path, content) {
                    Ok(_) => ToolResult {
                        success: true,
                        output: "File written".into(),
                        error: None,
                    },
                    Err(e) => ToolResult {
                        success: false,
                        output: String::new(),
                        error: Some(format!("Failed to write: {}", e)),
                    },
                }
            }
            _ => ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Unknown action: {}", action)),
            },
        }
    }
}

pub fn register_builtin_tools(registry: &mut crate::tools::ToolRegistry) {
    registry.register(DiskTool);
    registry.register(ShellTool);
    registry.register(FileTool);
}
