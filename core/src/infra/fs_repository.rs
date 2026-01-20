use crate::domain::{repository::RequestRepository, request::Request, project::Project};
use crate::infra::parser::parse_http_file;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct FsRequestRepository {
    root: PathBuf,
}

impl FsRequestRepository {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn resolve_path(&self, project: &Project, request_id: &str) -> PathBuf {
        self.root.join(&project.name).join(format!("{}.http", request_id))
    }
}

impl RequestRepository for FsRequestRepository {
    fn load(&self, project: &Project, request_id: &str) -> Result<Request> {
        let path = self.resolve_path(project, request_id);
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read request file: {:?}", path))?;
        parse_http_file(&content).with_context(|| format!("Failed to parse request file: {:?}", path))
    }

    fn save(&self, project: &Project, request_id: &str, request: &Request) -> Result<()> {
        let path = self.resolve_path(project, request_id);
        
        // Ensure project directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut content = format!("{} {}\n", request.method, request.url);
        
        for (key, value) in &request.headers {
            content.push_str(&format!("{}: {}\n", key, value));
        }
        
        content.push('\n');
        
        if let Some(body) = &request.body {
            content.push_str(body);
        }

        fs::write(&path, content)
            .with_context(|| format!("Failed to write request file: {:?}", path))?;
        Ok(())
    }
}
