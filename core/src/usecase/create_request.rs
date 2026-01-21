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

    pub fn execute(&self, project: &Project, request_id: &str, method: &str, body_type: &str) -> Result<()> {
        let mut content = format!("{} https://example.com", method);
        
        if body_type == "JSON" {
            content.push_str("\nContent-Type: application/json\n\n{\n    \n}");
        } else if body_type == "Empty" {
             // No body
        }

        // Just create the file with the content
        // We need to bypass Request::new() logic if we want to write raw string?
        // Wait, repository.save() takes a &Request.
        // And Parser parses a string to Request.
        // So we should construct a Request object.
        
        // Construct Request object
        let mut request = Request::new(method, "https://example.com");
        
        if body_type == "JSON" {
            request.headers.insert("Content-Type".to_string(), "application/json".to_string());
            request.body = Some("{\n    \n}".to_string());
        }

        self.repo.save(project, request_id, &request)
    }
}
