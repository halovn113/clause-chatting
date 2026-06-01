use serde::{Deserialize, Serialize};

pub mod builtin;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SafetyLevel {
    Safe,
    Careful,
    Dangerous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

pub trait ITool: Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str;

    fn is_read_only(&self) -> bool {
        false
    }

    fn safety_level(&self) -> SafetyLevel {
        SafetyLevel::Safe
    }

    fn execute(&self, args: serde_json::Value) -> ToolResult;
}

pub struct ToolRegistry {
    tools: Vec<Box<dyn ITool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register<T: ITool + 'static>(&mut self, tool: T) {
        self.tools.push(Box::new(tool));
    }

    pub fn get(&self, name: &str) -> Option<&dyn ITool> {
        self.tools
            .iter()
            .find(|t| t.name() == name)
            .map(|t| t.as_ref())
    }

    pub fn list(&self) -> Vec<ToolInfo> {
        self.tools
            .iter()
            .map(|t| ToolInfo {
                name: t.name().into(),
                description: t.description().into(),
                is_read_only: t.is_read_only(),
                safety_level: t.safety_level(),
            })
            .collect()
    }

    pub fn execute(&self, call: &ToolCall) -> ToolResult {
        match self.get(&call.name) {
            Some(tool) => tool.execute(call.arguments.clone()),
            None => ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Tool not found: {}", call.name)),
            },
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub is_read_only: bool,
    pub safety_level: SafetyLevel,
}
