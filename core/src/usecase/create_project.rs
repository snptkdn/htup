use crate::domain::repository::ProjectRepository;
use anyhow::Result;
use std::sync::Arc;

pub struct CreateProjectUseCase {
    repo: Arc<dyn ProjectRepository>,
}

impl CreateProjectUseCase {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { repo }
    }

    pub fn execute(&self, name: &str) -> Result<()> {
        self.repo.create_project(name)
    }
}
