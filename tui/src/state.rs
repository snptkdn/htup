use htup_core::domain::{project::Project, response::Response};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,
    ViewingResponse,
    CreatingProject,
    CreatingRequest, // Step 1: Name
    CreatingRequestMethod, // Step 2: Method
    CreatingRequestBody, // Step 3: Body Type
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FocusPane {
    Projects,
    Requests,
}

#[derive(Debug, Clone)]
pub struct PendingRequest {
    pub name: String,
    pub method: String,
    pub body_type: String,
}

pub struct AppState {
    pub mode: AppMode,
    pub focused_pane: FocusPane,
    pub projects: Vec<Project>,
    pub selected_project_index: usize,
    pub requests: Vec<String>,
    pub selected_request_index: usize,
    pub current_response: Option<Response>,
    pub status_message: Option<String>,
    pub input_buffer: String,
    
    // For Wizards
    pub pending_request: Option<PendingRequest>,
    pub selection_index: usize, // For Method/Body lists
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Normal,
            focused_pane: FocusPane::Projects,
            projects: Vec::new(),
            selected_project_index: 0,
            requests: Vec::new(),
            selected_request_index: 0,
            current_response: None,
            status_message: None,
            input_buffer: String::new(),
            pending_request: None,
            selection_index: 0,
        }
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.projects.get(self.selected_project_index)
    }

    pub fn selected_request_id(&self) -> Option<&str> {
        self.requests.get(self.selected_request_index).map(|s| s.as_str())
    }
}
