use crate::domain::{
    project::Project,
    repository::ProjectRepository,
};
use anyhow::Result;
use std::sync::Arc;

pub struct ListProjectsUseCase {
    repo: Arc<dyn ProjectRepository>,
}

impl ListProjectsUseCase {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { repo }
    }

    pub fn list_projects(&self) -> Result<Vec<Project>> {
        self.repo.list_projects()
    }

    pub fn list_requests(&self, project: &Project) -> Result<Vec<String>> {
        self.repo.list_requests(project)
    }
}
