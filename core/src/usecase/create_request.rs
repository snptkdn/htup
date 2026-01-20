use crate::domain::{
    project::Project,
    repository::RequestRepository,
    request::Request,
};
use anyhow::Result;
use std::sync::Arc;

pub struct CreateRequestUseCase {
    repo: Arc<dyn RequestRepository>,
}

impl CreateRequestUseCase {
    pub fn new(repo: Arc<dyn RequestRepository>) -> Self {
        Self { repo }
    }

    pub fn execute(&self, project: &Project, request_id: &str) -> Result<()> {
        // Default Template: Simple GET
        let request = Request::new("GET", "https://example.com");
        self.repo.save(project, request_id, &request)
    }
}
