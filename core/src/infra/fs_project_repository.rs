use crate::domain::{
    project::Project,
    repository::ProjectRepository,
};
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct FsProjectRepository {
    root: PathBuf,
}

impl FsProjectRepository {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

impl ProjectRepository for FsProjectRepository {
    fn list_projects(&self) -> Result<Vec<Project>> {
        let mut projects = Vec::new();
        if !self.root.exists() {
            fs::create_dir_all(&self.root)?;
        }

        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Project only stores name now
                    projects.push(Project::new(name));
                }
            }
        }
        projects.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(projects)
    }

    fn list_requests(&self, project: &Project) -> Result<Vec<String>> {
        let mut requests = Vec::new();
        // Construct the project path manually
        let project_path = self.root.join(&project.name);

        if project_path.exists() {
            for entry in fs::read_dir(&project_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "http" {
                            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                                requests.push(stem.to_string());
                            }
                        }
                    }
                }
            }
        }
        requests.sort();
        Ok(requests)
    }

    fn create_project(&self, name: &str) -> Result<()> {
        let path = self.root.join(name);
        if path.exists() {
            anyhow::bail!("Project already exists: {}", name);
        }
        fs::create_dir_all(&path).with_context(|| format!("Failed to create project directory: {:?}", path))?;
        Ok(())
    }
}
