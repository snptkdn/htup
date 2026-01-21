use crate::state::{AppMode, AppState, FocusPane};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    let main_area = chunks[0];
    let status_area = chunks[1];

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(main_area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_chunks[0]);

    draw_projects(f, state, left_chunks[0]);
    draw_requests(f, state, left_chunks[1]);
    draw_content(f, state, main_chunks[1]);
    draw_status_bar(f, state, status_area);

    // Draw Popups overlay
    if match state.mode {
        AppMode::CreatingProject | AppMode::CreatingRequest | AppMode::CreatingRequestMethod | AppMode::CreatingRequestBody => true,
        _ => false,
    } {
        draw_input_popup(f, state, f.size());
    }
}

fn draw_projects(f: &mut Frame, state: &AppState, area: Rect) {
    let items: Vec<ListItem> = state
        .projects
        .iter()
        .map(|p| ListItem::new(p.name.as_str()))
        .collect();

    let border_color = if let FocusPane::Projects = state.focused_pane {
        Color::Green
    } else {
        Color::White
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Projects").border_style(Style::default().fg(border_color)))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_project_index));

    f.render_stateful_widget(list, area, &mut list_state);
}

fn draw_requests(f: &mut Frame, state: &AppState, area: Rect) {
    let items: Vec<ListItem> = state
        .requests
        .iter()
        .map(|r| ListItem::new(r.as_str()))
        .collect();

    let border_color = if let FocusPane::Requests = state.focused_pane {
        Color::Green
    } else {
        Color::White
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Requests").border_style(Style::default().fg(border_color)))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan))
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_request_index));

    f.render_stateful_widget(list, area, &mut list_state);
}

fn draw_content(f: &mut Frame, state: &AppState, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Content");
    
    match state.mode {
        AppMode::ViewingResponse => {
            if let Some(resp) = &state.current_response {
                let status_line = Line::from(vec![
                    Span::raw(format!("Status: {} {} | Time: {:?}", resp.status, resp.status_text, resp.latency)),
                ]);
                
                let body_lines: Vec<Line> = resp.body.lines().map(Line::from).collect();
                let mut content = vec![status_line, Line::from("")];
                content.extend(body_lines);

                let p = Paragraph::new(content).block(block).wrap(Wrap { trim: false });
                f.render_widget(p, area);
            } else {
                 f.render_widget(Paragraph::new("No response").block(block), area);
            }
        }
        _ => {
            f.render_widget(Paragraph::new("Press <Enter> to run request\nPress <n> to create new Request\nPress <N> (shift+n) to create new Project").block(block), area);
        }
    }
}

fn draw_status_bar(f: &mut Frame, state: &AppState, area: Rect) {
    let msg = state.status_message.as_deref().unwrap_or("Ready");
    let p = Paragraph::new(msg).style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(p, area);
}

fn draw_input_popup(f: &mut Frame, state: &AppState, area: Rect) {
    let title = match state.mode {
        AppMode::CreatingProject => "Create New Project",
        AppMode::CreatingRequest => "Request Name",
        AppMode::CreatingRequestMethod => "Select Method",
        AppMode::CreatingRequestBody => "Select Body Type",
        _ => "",
    };

    let block = Block::default().borders(Borders::ALL).title(title);
    let area = centered_rect(60, 40, area); // Increased height for lists
    f.render_widget(ratatui::widgets::Clear, area); // Clear background
    f.render_widget(block.clone(), area);

    let inner_area = block.inner(area);

    match state.mode {
        AppMode::CreatingProject | AppMode::CreatingRequest => {
             let input = Paragraph::new(state.input_buffer.as_str())
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(input, inner_area);
        }
        AppMode::CreatingRequestMethod => {
            let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH"];
            let items: Vec<ListItem> = methods.iter().map(|m| ListItem::new(*m)).collect();
            let list = List::new(items)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
                .highlight_symbol("> ");
            
            let mut list_state = ListState::default();
            list_state.select(Some(state.selection_index));
            f.render_stateful_widget(list, inner_area, &mut list_state);
        }
        AppMode::CreatingRequestBody => {
            let types = vec!["Empty", "JSON"];
            let items: Vec<ListItem> = types.iter().map(|t| ListItem::new(*t)).collect();
            let list = List::new(items)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
                .highlight_symbol("> ");
            
            let mut list_state = ListState::default();
            list_state.select(Some(state.selection_index));
            f.render_stateful_widget(list, inner_area, &mut list_state);
        }
        _ => {}
    }
}

/// Helper to center a rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
