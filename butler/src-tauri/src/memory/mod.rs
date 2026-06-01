use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub key: String,
    pub value: String,
    pub category: String,
    pub created_at: String,
}

pub struct MemoryManager {
    base_dir: PathBuf,
}

impl MemoryManager {
    pub fn new(base_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&base_dir)
            .context("Failed to create memory directory")?;

        fs::create_dir_all(base_dir.join("projects"))
            .context("Failed to create projects directory")?;

        fs::create_dir_all(base_dir.join("preferences"))
            .context("Failed to create preferences directory")?;

        fs::create_dir_all(base_dir.join("habits"))
            .context("Failed to create habits directory")?;

        fs::create_dir_all(base_dir.join("computer"))
            .context("Failed to create computer directory")?;

        Ok(Self { base_dir })
    }

    pub fn read_memory(&self, category: &str) -> Result<String> {
        let dir = self.base_dir.join(category);
        let mut content = String::new();

        if dir.exists() {
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    let file_content =
                        fs::read_to_string(&path).unwrap_or_default();
                    let name = path
                        .file_stem()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    content.push_str(&format!("# {}\n\n{}\n\n", name, file_content));
                }
            }
        }

        Ok(content)
    }

    pub fn write_memory(
        &self,
        category: &str,
        key: &str,
        content: &str,
    ) -> Result<()> {
        let dir = self.base_dir.join(category);
        fs::create_dir_all(&dir)?;

        let path = dir.join(format!("{}.md", sanitize_filename(key)));
        fs::write(&path, content)?;

        Ok(())
    }

    pub fn delete_memory(&self, category: &str, key: &str) -> Result<()> {
        let path = self
            .base_dir
            .join(category)
            .join(format!("{}.md", sanitize_filename(key)));

        if path.exists() {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    pub fn list_entries(&self, category: &str) -> Result<Vec<String>> {
        let dir = self.base_dir.join(category);
        let mut entries = Vec::new();

        if dir.exists() {
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    entries.push(
                        path.file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default(),
                    );
                }
            }
        }

        Ok(entries)
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}
