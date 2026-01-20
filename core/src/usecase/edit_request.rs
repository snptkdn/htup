use crate::domain::{
    project::Project,
    repository::Editor,
};
use anyhow::Result;
use std::sync::Arc;

pub struct EditRequestUseCase {
    editor: Arc<dyn Editor>,
}

impl EditRequestUseCase {
    pub fn new(editor: Arc<dyn Editor>) -> Self {
        Self { editor }
    }

    pub fn execute(&self, project: &Project, request_id: &str) -> Result<()> {
        self.editor.edit(project, request_id)
    }
}
