use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use lilicore::{
    git_repo::{get_current_branch_name, git_temporary_branch_create},
    shell::run_shell_command,
};
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
                // update current_branch in the state
                let current_branch_name = get_current_branch_name(&state.project_dir)?;
                if current_branch_name.clone().starts_with("temp-") {
                    anyhow::bail!("You are already on a temporary branch");
                }
                state.set_base_branch_name(&current_branch_name)?;
                // create temporary branch
                match git_temporary_branch_create(&state.project_dir) {
                    Ok(output) => {
                        state.set_screen(AppScreen::Mission);
                        state.set_focused_block(FocusedBlock::Home);
                        state.set_header_status(HeaderStatus::SuccessMessage(output));
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                    Err(err) => {
                        state.set_header_status(HeaderStatus::ErrorMessage(err.to_string()));
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                };
                return Ok(ShortcutHandlerResponse::StopPropagation);
            }
            KeyCode::Esc => {
                state.set_screen(AppScreen::Mission);
                state.set_focused_block(FocusedBlock::Home);
                return Ok(ShortcutHandlerResponse::Mission);
            }
            KeyCode::Char('q') => {
                return Ok(ShortcutHandlerResponse::Exit);
            }
            _ => {}
        }
        Ok(ShortcutHandlerResponse::StopPropagation)
    }
}

fn _run_shell_command(command: &str, project_dir: &str) -> Result<String> {
    let res = run_shell_command(command, project_dir);
    if !res.stderr.is_empty() {
        anyhow::bail!(format!("error: {}", res.stderr));
    }
    Ok(res.stdout)
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

        let [line1_rect, line2_rect, line3_rect, bottom_line_rect] = *Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(_main_rect)
            else {
                return Ok(HashMap::new());
            };

        let project_dir = state.project_dir.clone();
        let current_branch = get_current_branch_name(&project_dir)?;

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
            (
                bottom_line_rect,
                Line::from(Span::raw(format!("current branch: {}", current_branch))),
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
