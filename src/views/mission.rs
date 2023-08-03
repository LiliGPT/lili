use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, Frame};

use crate::{
    app::{AppState, FocusedBlock},
    components::{
        actions::ActionsComponent, context_files::ContextFilesComponent, header::HeaderComponent,
        message_input::MessageInputComponent, project_info::ProjectInfoComponent,
        shortcuts::ShortcutsComponent, text_input::TextInputComponent, AppComponent,
    },
    shortcuts::{handle_text_input_event, ShortcutHandlerResponse},
};

use super::AppViewTrait;

pub struct MissionView;

impl MissionView {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppViewTrait for MissionView {
    fn components(&mut self, state: &mut AppState) -> Result<HashMap<String, Mutex<AppComponent>>> {
        let el_message = MessageInputComponent::new()?;
        let el_header = HeaderComponent::new("Mission Control".to_string())?;
        let el_context_files = ContextFilesComponent::new()?;
        let el_actions = ActionsComponent::new()?;
        let el_shortcuts = ShortcutsComponent::from_focused_block(state.focused_block.clone())?;
        let el_project_info = ProjectInfoComponent::new(state.project_dir.clone())?;

        let mut components = HashMap::new();
        components.insert(String::from("message"), el_message.as_mutex());
        components.insert(String::from("header"), el_header.as_mutex());
        components.insert(String::from("context_files"), el_context_files.as_mutex());
        components.insert(String::from("actions"), el_actions.as_mutex());
        components.insert(String::from("shortcuts"), el_shortcuts.as_mutex());
        components.insert(String::from("project_info"), el_project_info.as_mutex());

        Ok(components)
    }

    fn positions<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        state: &mut AppState,
    ) -> Result<HashMap<String, Rect>> {
        let [top_rect, _main_rect, bottom_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(4),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(frame.size())
            else {
                return Ok(HashMap::new());
            };

        let [_left_rect, right_rect] = *Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(0)
            .vertical_margin(0)
            .constraints([Constraint::Ratio(2, 6), Constraint::Ratio(4, 6)].as_ref())
            .split(_main_rect)
            else {
                return Ok(HashMap::new());
            };

        let [left_top_rect, left_mid_rect, left_bottom_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ].as_ref())
            .split(_left_rect)
            else {
                return Ok(HashMap::new());
            };

        let positions = vec![
            (String::from("header"), top_rect),
            (String::from("message"), left_top_rect),
            (String::from("context_files"), left_mid_rect),
            (String::from("actions"), left_bottom_rect),
            (String::from("project_info"), right_rect),
            (String::from("shortcuts"), bottom_rect),
        ];

        Ok(positions.into_iter().collect())
    }

    fn handle_events(
        &mut self,
        state: &mut AppState,
        key: &KeyEvent,
    ) -> Result<ShortcutHandlerResponse> {
        match &state.focused_block {
            FocusedBlock::Message => {
                if let KeyCode::Enter = key.code {
                    state.set_focused_block(FocusedBlock::Home);
                    state.set_input_value("message", "");
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                return handle_text_input_event(state, key, &MessageInputComponent::unique_name());
            }
            _ => {}
        }

        match key.code {
            KeyCode::Char('i') => {
                state.set_focused_block(FocusedBlock::Message);
                Ok(ShortcutHandlerResponse::StopPropagation)
            }
            _ => Ok(ShortcutHandlerResponse::Continue),
        }
    }
}
