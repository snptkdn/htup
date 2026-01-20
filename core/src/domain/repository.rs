use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use super::request::Request;
use super::response::Response;
use super::project::Project;
use anyhow::Result;

/// Repository for managing Projects.
#[cfg_attr(test, automock)]
pub trait ProjectRepository: Send + Sync {
    /// Lists all available projects.
    fn list_projects(&self) -> Result<Vec<Project>>;
    /// Lists all request IDs (names) in a project.
    fn list_requests(&self, project: &Project) -> Result<Vec<String>>;
    /// Creates a new project (directory).
    fn create_project(&self, name: &str) -> Result<()>;
}

/// Repository for loading and saving Requests.
#[cfg_attr(test, automock)]
pub trait RequestRepository: Send + Sync {
    /// Loads a request by ID within a project.
    fn load(&self, project: &Project, request_id: &str) -> Result<Request>;
    /// Saves a request by ID within a project.
    fn save(&self, project: &Project, request_id: &str, request: &Request) -> Result<()>;
}

/// Gateway for sending HTTP requests.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait HttpClient: Send + Sync {
    /// Sends the request and returns the response.
    async fn send(&self, request: &Request) -> Result<Response>;
}

/// Gateway for interacting with an external editor.
#[cfg_attr(test, automock)]
pub trait Editor: Send + Sync {
    /// Opens a request file for editing.
    /// Note: This still maps to a physical file, so the implementation needs to resolve it.
    /// Ideally the Editor trait should perhaps take a Project + RequestId too, 
    /// but for now let's abstract it: `edit(&self, project: &Project, request_id: &str)`.
    fn edit(&self, project: &Project, request_id: &str) -> Result<()>;
}
