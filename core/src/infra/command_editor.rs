use crate::domain::{repository::Editor, project::Project};
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub struct SystemCommandEditor {
    root: PathBuf,
}

impl SystemCommandEditor {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn resolve_path(&self, project: &Project, request_id: &str) -> PathBuf {
        self.root.join(&project.name).join(format!("{}.http", request_id))
    }
}

impl Editor for SystemCommandEditor {
    fn edit(&self, project: &Project, request_id: &str) -> Result<()> {
        let path = self.resolve_path(project, request_id);
        
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
        
        // Ensure parent dir exists so editor doesn't fail on new file
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let status = Command::new(&editor)
            .arg(&path)
            .status()
            .with_context(|| format!("Failed to launch editor: {}", editor))?;

        if !status.success() {
            anyhow::bail!("Editor exited with non-zero status code");
        }

        Ok(())
    }
}
