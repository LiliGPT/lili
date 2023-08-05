use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Backend, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Padding, Paragraph},
    Frame,
};

use crate::{
    app::{AppScreen, AppState, FocusedBlock},
    components::{
        header::{HeaderComponent, HeaderStatus},
        shortcuts::ShortcutsComponent,
        AppComponent,
    },
    redraw_app,
    shortcuts::ShortcutHandlerResponse,
};

use super::AppViewTrait;

pub struct CreateTempBranchView;

impl CreateTempBranchView {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle_events(
        &mut self,
        state: &mut AppState,
        key: &KeyEvent,
    ) -> Result<ShortcutHandlerResponse> {
        match key.code {
            KeyCode::Enter => {
                // state.set_screen(AppScreen::Mission);
                // state.set_focused_block(FocusedBlock::Home);
                // return Ok(ShortcutHandlerResponse::Mission);
                match git_temporary_branch_create(state) {
                    Ok(output) => {
                        state.set_header_status(HeaderStatus::LoadingMessage(output));
                    }
                    Err(err) => {
                        state.set_header_status(HeaderStatus::ErrorMessage(err.to_string()));
                    }
                };
                return Ok(ShortcutHandlerResponse::StopPropagation);
            }
            KeyCode::Esc => {
                // return Ok(ShortcutHandlerResponse::Continue);
                match git_temporary_branch_destroy() {
                    Ok(output) => {
                        state.set_header_status(HeaderStatus::LoadingMessage(output));
                    }
                    Err(err) => {
                        state.set_header_status(HeaderStatus::ErrorMessage(err.to_string()));
                    }
                };
                return Ok(ShortcutHandlerResponse::StopPropagation);
            }
            KeyCode::Char('q') => {
                return Ok(ShortcutHandlerResponse::Exit);
            }
            _ => {}
        }
        Ok(ShortcutHandlerResponse::StopPropagation)
    }
}

fn git_temporary_branch_create(state: &mut AppState) -> Result<String> {
    let now_str = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs()
        .to_string();
    let command = format!("git checkout -b temp-{}", now_str);
    let project_dir = state.project_dir.clone();
    let output = std::process::Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(format!("temp-{}", now_str))
        .current_dir(project_dir)
        .output()?;
    if !output.status.clone().success() {
        let error_message = String::from_utf8(output.stderr.clone())?;
        anyhow::bail!(error_message);
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn git_temporary_branch_destroy() -> Result<String> {
    let command = format!("git reset --soft HEAD~1");
    let output = std::process::Command::new("git")
        .arg("reset")
        .arg("--soft")
        .arg("HEAD~1")
        .output()?;
    if !output.status.success() {
        let error_message = String::from_utf8(output.stderr)?;
        anyhow::bail!(error_message);
    }
    Ok(String::from_utf8(output.stderr)?)
}

impl AppViewTrait for CreateTempBranchView {
    fn components(&mut self, state: &mut AppState) -> Result<HashMap<String, Mutex<AppComponent>>> {
        let el_header = HeaderComponent::new()?;
        let el_shortcuts = ShortcutsComponent::new()?;

        let mut components = HashMap::new();
        components.insert(String::from("header"), el_header.as_mutex());
        components.insert(String::from("shortcuts"), el_shortcuts.as_mutex());

        Ok(components)
    }

    fn positions<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        state: &mut AppState,
    ) -> Result<HashMap<String, Rect>> {
        let [top_rect, _main_rect, bottom_rect] = *Layout::default()
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(frame.size())
            else {
                return Ok(HashMap::new());
            };

        let [line1_rect, line2_rect, line3_rect] = *Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(_main_rect)
            else {
                return Ok(HashMap::new());
            };

        let line_contents: Vec<(Rect, Line)> = vec![
            (
                line1_rect,
                Line::from(Span::raw("Would you like to create a temporary branch?")),
            ),
            (
                line2_rect,
                Line::from(Span::raw("\n\nThis is highly recommended!")),
            ),
            (
                line3_rect,
                Line::from(vec![
                    Span::raw("[Enter]"),
                    Span::styled(
                        " yes",
                        ratatui::style::Style::default().fg(ratatui::style::Color::Green),
                    ),
                    Span::raw("   "),
                    Span::raw("[Esc]"),
                    Span::styled(
                        " no",
                        ratatui::style::Style::default().fg(ratatui::style::Color::Red),
                    ),
                ]),
            ),
        ];

        let positions = vec![
            (String::from("header"), top_rect),
            (String::from("shortcuts"), bottom_rect),
        ];

        for line in line_contents {
            let block = Paragraph::new(line.1)
                .block(
                    ratatui::widgets::Block::default()
                        .borders(ratatui::widgets::Borders::NONE)
                        .padding(Padding::new(2, 2, 2, 0)),
                )
                .alignment(ratatui::prelude::Alignment::Center);
            frame.render_widget(block, line.0);
        }

        Ok(positions.into_iter().collect())
    }
}
