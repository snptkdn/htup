use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, sync::Arc, time::Duration};

mod app;
mod state;
mod ui;

use app::App;
use htup_core::{
    infra::{
        fs_project_repository::FsProjectRepository,
        fs_repository::FsRequestRepository,
        reqwest_client::ReqwestHttpClient,
    },
    usecase::{
        execute_request::ExecuteRequestUseCase,
        list_projects::ListProjectsUseCase,
        create_project::CreateProjectUseCase,
        create_request::CreateRequestUseCase,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup Dependencies (DI)
    let root_dir = std::env::current_dir()?;
    let project_repo = Arc::new(FsProjectRepository::new(root_dir.clone()));
    let request_repo = Arc::new(FsRequestRepository::new(root_dir.clone()));
    let command_editor = Arc::new(htup_core::infra::command_editor::SystemCommandEditor::new(root_dir));
    let http_client = Arc::new(ReqwestHttpClient::new());

    // Setup UseCases
    let list_projects = ListProjectsUseCase::new(project_repo.clone());
    let execute_request = ExecuteRequestUseCase::new(http_client);
    let create_project = CreateProjectUseCase::new(project_repo);
    let create_request = CreateRequestUseCase::new(request_repo.clone());
    let edit_request = htup_core::usecase::edit_request::EditRequestUseCase::new(command_editor);

    // Setup App
    let mut app = App::new(
        list_projects, 
        execute_request, 
        create_project, 
        create_request, 
        edit_request,
        request_repo
    );
    app.init().await?;

    // Run Event Loop
    let res = run_app(&mut terminal, &mut app).await;

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend + std::io::Write>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &mut app.state))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match app.state.mode {
                    // Input Mode handling
                    state::AppMode::CreatingProject | state::AppMode::CreatingRequest | state::AppMode::CreatingRequestMethod | state::AppMode::CreatingRequestBody => {
                        match key.code {
                            KeyCode::Enter => app.on_enter().await?,
                            KeyCode::Esc => app.on_esc(),
                            KeyCode::Backspace => app.on_backspace(),
                            KeyCode::Char(c) => app.on_char(c),
                            KeyCode::Up => app.on_up(),
                            KeyCode::Down => app.on_down(),
                            _ => {}
                        }
                    }
                    // Normal Navigation
                    _ => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        // Navigation
                        KeyCode::Char('j') => app.next(),
                        KeyCode::Char('k') => app.previous(),
                        // Focus Switching
                        KeyCode::Tab => app.switch_focus(),
                        KeyCode::Char('h') => app.focus_projects(),
                        KeyCode::Char('l') => app.focus_requests(),
                        
                        // Creation
                        KeyCode::Char('N') => app.start_create_project(),
                        KeyCode::Char('n') => app.start_create_request(),

                        // Execution
                        KeyCode::Enter => app.on_enter().await?,
                        KeyCode::Esc => app.on_esc(),
                        
                        // Editing (Important: Suspend Terminal)
                        KeyCode::Char('e') => {
                            // Restore terminal for editor
                            execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
                            disable_raw_mode()?;
                            terminal.show_cursor()?;

                            // Run Editor
                            if let Err(e) = app.on_edit() {
                                app.state.status_message = Some(format!("Edit failed: {}", e));
                            }

                            // Re-enable terminal
                            enable_raw_mode()?;
                            execute!(terminal.backend_mut(), EnterAlternateScreen, EnableMouseCapture)?;
                            terminal.hide_cursor()?;
                            terminal.clear()?;
                            // Force redraw immediately
                            terminal.draw(|f| ui::draw(f, &mut app.state))?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
