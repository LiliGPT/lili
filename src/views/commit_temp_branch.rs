use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use lilicore::shell::run_shell_command;
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
        text_input::TextInputComponent,
        AppComponent,
    },
    shortcuts::{handle_text_input_event, ShortcutHandlerResponse},
};

use super::AppViewTrait;

pub struct CommitTempBranchView;

impl CommitTempBranchView {
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
                let message = &state.get_input_value_from_focused(FocusedBlock::CommitMessage);

                if message.is_empty() {
                    state.set_header_status(HeaderStatus::ErrorMessage(
                        "Please enter a commit message".to_string(),
                    ));
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }

                let base_branch_name = "master";
                let project_dir = &state.project_dir.clone();
                match git_temporary_branch_destroy(base_branch_name, project_dir) {
                    Ok(output) => {
                        state.set_header_status(HeaderStatus::SuccessMessage(output));
                    }
                    Err(err) => {
                        state.set_header_status(HeaderStatus::ErrorMessage(err.to_string()));
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                };
                match git_add_and_commit(message, project_dir) {
                    Ok(output) => {
                        state.set_header_status(HeaderStatus::SuccessMessage(output));
                        return Ok(ShortcutHandlerResponse::Mission);
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
            _ => handle_text_input_event(state, key, &FocusedBlock::CommitMessage),
        }
    }
}

fn git_get_current_branch(project_dir: &str) -> Result<String> {
    let project_dir = project_dir.to_string();
    let output = std::process::Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .current_dir(project_dir)
        .output()?;
    if !output.status.clone().success() {
        let error_message = String::from_utf8(output.stderr.clone())?;
        anyhow::bail!(error_message);
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn git_temporary_branch_destroy(base_branch_name: &str, project_dir: &str) -> Result<String> {
    // todo: save branch name in state
    let temp_branch_name = &git_get_current_branch(project_dir)?;
    if !temp_branch_name.starts_with("temp-") {
        anyhow::bail!("not a temp branch");
    }
    _run_shell_command(
        &format!("git reset --soft {}", base_branch_name),
        project_dir,
    )?;
    _run_shell_command(&format!("git checkout {}", base_branch_name), project_dir)
        .unwrap_or(String::new());
    _run_shell_command(&format!("git branch -d {}", temp_branch_name), project_dir)
}

fn git_add_and_commit(message: &str, project_dir: &str) -> Result<String> {
    _run_shell_command("git add .", project_dir)?;
    _run_shell_command(&format!("git commit -m \"{}\"", message), project_dir)
}

fn _run_shell_command(command: &str, project_dir: &str) -> Result<String> {
    let res = run_shell_command(command, project_dir);
    if !res.stderr.is_empty() {
        anyhow::bail!(format!("error: {}", res.stderr));
    }
    Ok(res.stdout)
}

impl AppViewTrait for CommitTempBranchView {
    fn components(&mut self, state: &mut AppState) -> Result<HashMap<String, Mutex<AppComponent>>> {
        let el_header = HeaderComponent::new()?;
        let el_shortcuts = ShortcutsComponent::new()?;
        let el_message = TextInputComponent::new("commit message", FocusedBlock::CommitMessage)?;

        let mut components = HashMap::new();
        components.insert(String::from("header"), el_header.as_mutex());
        components.insert(String::from("shortcuts"), el_shortcuts.as_mutex());
        components.insert(String::from("message"), el_message.as_mutex());

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
                Line::from(Span::raw(
                    "Would you like to pack all your changes into a single commit?",
                )),
            ),
            (line3_rect, Line::from(Span::raw("[Enter] yes   [Esc] no"))),
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

        let positions = vec![
            (String::from("header"), top_rect),
            (String::from("shortcuts"), bottom_rect),
            (String::from("message"), line2_rect),
        ];

        Ok(positions.into_iter().collect())
    }
}
