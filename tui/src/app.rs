use crate::state::{AppMode, AppState, FocusPane};
use anyhow::Result;
use htup_core::{
    domain::{repository::ProjectRepository, request::Request},
    usecase::{
        execute_request::ExecuteRequestUseCase, 
        list_projects::ListProjectsUseCase,
        create_project::CreateProjectUseCase,
        create_request::CreateRequestUseCase,
        edit_request::EditRequestUseCase,
    },
};
use std::path::PathBuf;
use std::sync::Arc;

pub struct App {
    pub state: AppState,
    list_projects_usecase: ListProjectsUseCase,
    execute_request_usecase: ExecuteRequestUseCase,
    create_project_usecase: CreateProjectUseCase,
    create_request_usecase: CreateRequestUseCase,
    edit_request_usecase: EditRequestUseCase,
    request_repo: Arc<dyn htup_core::domain::repository::RequestRepository>,
}

impl App {
    pub fn new(
        list_projects_usecase: ListProjectsUseCase,
        execute_request_usecase: ExecuteRequestUseCase,
        create_project_usecase: CreateProjectUseCase,
        create_request_usecase: CreateRequestUseCase,
        edit_request_usecase: EditRequestUseCase,
        request_repo: Arc<dyn htup_core::domain::repository::RequestRepository>,
    ) -> Self {
        Self {
            state: AppState::new(),
            list_projects_usecase,
            execute_request_usecase,
            create_project_usecase,
            create_request_usecase,
            edit_request_usecase,
            request_repo,
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        self.refresh_projects()?;
        Ok(())
    }

    pub fn refresh_projects(&mut self) -> Result<()> {
        self.state.projects = self.list_projects_usecase.list_projects()?;
        if !self.state.projects.is_empty() {
            self.refresh_requests()?;
        }
        Ok(())
    }

    pub fn refresh_requests(&mut self) -> Result<()> {
        if let Some(project) = self.state.selected_project() {
            self.state.requests = self.list_projects_usecase.list_requests(project)?;
            // Reset selection if out of bounds
            if self.state.selected_request_index >= self.state.requests.len() {
                self.state.selected_request_index = 0;
            }
        }
        Ok(())
    }

    pub fn next(&mut self) {
        match self.state.focused_pane {
            FocusPane::Projects => self.next_project(),
            FocusPane::Requests => self.next_request(),
        }
    }

    pub fn previous(&mut self) {
        match self.state.focused_pane {
            FocusPane::Projects => self.previous_project(),
            FocusPane::Requests => self.previous_request(),
        }
    }

    fn next_project(&mut self) {
        if !self.state.projects.is_empty() {
            self.state.selected_project_index = (self.state.selected_project_index + 1) % self.state.projects.len();
            self.refresh_requests().unwrap_or_default();
        }
    }

    fn previous_project(&mut self) {
        if !self.state.projects.is_empty() {
            if self.state.selected_project_index == 0 {
                self.state.selected_project_index = self.state.projects.len() - 1;
            } else {
                self.state.selected_project_index -= 1;
            }
            self.refresh_requests().unwrap_or_default();
        }
    }

    fn next_request(&mut self) {
        if !self.state.requests.is_empty() {
            self.state.selected_request_index = (self.state.selected_request_index + 1) % self.state.requests.len();
        }
    }

    fn previous_request(&mut self) {
        if !self.state.requests.is_empty() {
            if self.state.selected_request_index == 0 {
                self.state.selected_request_index = self.state.requests.len() - 1;
            } else {
                self.state.selected_request_index -= 1;
            }
        }
    }

    pub fn switch_focus(&mut self) {
        self.state.focused_pane = match self.state.focused_pane {
            FocusPane::Projects => FocusPane::Requests,
            FocusPane::Requests => FocusPane::Projects,
        };
    }

    pub fn focus_projects(&mut self) {
        self.state.focused_pane = FocusPane::Projects;
    }

    pub fn focus_requests(&mut self) {
        self.state.focused_pane = FocusPane::Requests;
    }

    pub async fn on_enter(&mut self) -> Result<()> {
        match self.state.mode {
            AppMode::Normal => {
                match self.state.focused_pane {
                    FocusPane::Projects => {
                         // Enter on project list switches focus to requests (common pattern)
                         self.focus_requests();
                    }
                    FocusPane::Requests => {
                        // Execute Request
                        // Clone data first to avoid borrow conflicts
                        let execution_target = if let (Some(project), Some(req_id)) = (self.state.selected_project(), self.state.selected_request_id()) {
                            Some((project.clone(), req_id.to_string()))
                        } else {
                            None
                        };

                        if let Some((project, req_id)) = execution_target {
                            // Now safe to mutate state
                            self.state.status_message = Some(format!("Executing {}...", req_id));
                            self.state.current_response = None; // Clear previous response
                            
                            // Load request (might fail, so handle error)
                            match self.request_repo.load(&project, &req_id) {
                                Ok(request) => {
                                    match self.execute_request_usecase.execute(&request).await {
                                        Ok(response) => {
                                            self.state.current_response = Some(response);
                                            self.state.status_message = Some(format!("Executed '{}' successfully", req_id));
                                            self.state.mode = AppMode::ViewingResponse;
                                        }
                                        Err(e) => {
                                            self.state.status_message = Some(format!("Error: {}", e));
                                        }
                                    }
                                }
                                Err(e) => {
                                      self.state.status_message = Some(format!("Failed to load request: {}", e));
                                }
                            }
                        }
                    }
                }
            }
            AppMode::CreatingProject => {
                let name = self.state.input_buffer.clone();
                if !name.is_empty() {
                    self.create_project_usecase.execute(&name)?;
                    self.state.mode = AppMode::Normal;
                    self.state.input_buffer.clear();
                    self.refresh_projects()?;
                    self.state.status_message = Some(format!("Project '{}' created", name));
                }
            }
            AppMode::CreatingRequest => {
                let name = self.state.input_buffer.clone();
                 if let Some(project) = self.state.selected_project().cloned() {
                    if !name.is_empty() {
                        self.create_request_usecase.execute(&project, &name)?;
                        self.state.mode = AppMode::Normal;
                        self.state.input_buffer.clear();
                        self.refresh_requests()?;
                        self.state.status_message = Some(format!("Request '{}' created in '{}'", name, project.name));
                        // Automatically open editor after creation
                        self.on_edit()?;
                    }
                 }
            }
            AppMode::ViewingResponse => {
                // Return to normal? Or maybe toggle body/headers
            }
            _ => {}
        }
        Ok(())
    }

    pub fn on_edit(&mut self) -> Result<()> {
        if let (Some(project), Some(req_id)) = (self.state.selected_project(), self.state.selected_request_id()) {
             // We need to suspend the terminal before opening the editor?
             // Actually ratatui/crossterm might need cleanup.
             // But since we are using SystemCommandEditor which uses std::process::Command with inherited stdio/stdout...
             // We might need to temporarily leave raw mode in main.rs, not here.
             // But the UseCase just calls the command.
             // Let's rely on main.rs handling the logic or do it here if we pass a callback?
             // For simplicity, the UseCase runs the command.
             // If the command is interactive (vim), it needs terminal control.
             // So we should probably handle this in main.rs by checking a flag or return value?
             // OR, we can just run it. But running vim inside raw mode might be weird.
             // Ideally we signal main loop to suspend.
             // For now, let's just call the usecase and see if we can handle the terminal in main.rs logic via `AppMode`.
             self.edit_request_usecase.execute(project, req_id)?;
             self.state.status_message = Some(format!("Edited {}", req_id));
        }
        Ok(())
    }

    pub fn on_esc(&mut self) {
        match self.state.mode {
            AppMode::ViewingResponse => {
                self.state.mode = AppMode::Normal;
            }
            AppMode::CreatingProject | AppMode::CreatingRequest => {
                self.state.mode = AppMode::Normal;
                self.state.input_buffer.clear();
            }
            _ => {
                // Maybe quit?
            }
        }
    }

    pub fn on_char(&mut self, c: char) {
        match self.state.mode {
            AppMode::CreatingProject | AppMode::CreatingRequest => {
                self.state.input_buffer.push(c);
            }
            _ => {}
        }
    }

    pub fn on_backspace(&mut self) {
         match self.state.mode {
            AppMode::CreatingProject | AppMode::CreatingRequest => {
                self.state.input_buffer.pop();
            }
            _ => {}
        }
    }

    pub fn start_create_project(&mut self) {
        self.state.mode = AppMode::CreatingProject;
        self.state.input_buffer.clear();
    }

    pub fn start_create_request(&mut self) {
        if self.state.selected_project().is_some() {
            self.state.mode = AppMode::CreatingRequest;
            self.state.input_buffer.clear();
        } else {
            self.state.status_message = Some("No project selected".to_string());
        }
    }
}
