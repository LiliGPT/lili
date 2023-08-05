use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use lilicore::{
    code_analyst,
    code_missions_api::{
        self, ApiError, CreateMissionRequest, CreateMissionResponse, ExecuteMissionRequest,
        MissionData, MissionExecution, MissionExecutionContextFile,
    },
    io::LocalPath,
};
use ratatui::{prelude::*, Frame};

use crate::{
    app::{AppState, FocusedBlock},
    components::{
        header::{HeaderComponent, HeaderStatus},
        mission::{
            action_preview::ActionPreviewComponent, actions::ActionsComponent,
            context_files::ContextFilesComponent, message_input::MessageInputComponent,
            project_info::ProjectInfoComponent,
        },
        shortcuts::ShortcutsComponent,
        AppComponent,
    },
    shortcuts::{handle_text_input_event, ShortcutHandlerResponse},
};

use super::AppViewTrait;

pub struct MissionView;

impl MissionView {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle_events(
        &mut self,
        state: &mut AppState,
        key: &KeyEvent,
    ) -> Result<ShortcutHandlerResponse> {
        match &state.focused_block {
            FocusedBlock::Message => {
                if let KeyCode::Enter = key.code {
                    // state.set_focused_block(FocusedBlock::Home);
                    // state.set_input_value("message", "");
                    // return Ok(ShortcutHandlerResponse::StopPropagation);
                    return self.send_message(state).await;
                }
                return handle_text_input_event(state, key, &FocusedBlock::Message);
            }
            FocusedBlock::ContextFiles => match key.code {
                KeyCode::Up => {
                    state.context_items.select_previous();
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                KeyCode::Down => {
                    state.context_items.select_next();
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                _ => {}
            },
            FocusedBlock::Actions => match key.code {
                KeyCode::Up => {
                    state.action_items.select_previous();
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                KeyCode::Down => {
                    state.action_items.select_next();
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                _ => {}
            },
            _ => {}
        }

        match key.code {
            KeyCode::Char('i') => {
                state.set_focused_block(FocusedBlock::Message);
                Ok(ShortcutHandlerResponse::StopPropagation)
            }
            KeyCode::Char('c') => {
                state.set_focused_block(FocusedBlock::ContextFiles);
                Ok(ShortcutHandlerResponse::StopPropagation)
            }
            KeyCode::Char('a') => {
                state.set_focused_block(FocusedBlock::Actions);
                Ok(ShortcutHandlerResponse::StopPropagation)
            }
            _ => Ok(ShortcutHandlerResponse::Continue),
        }
    }

    async fn send_message(&mut self, state: &mut AppState) -> Result<ShortcutHandlerResponse> {
        let pathinfo = code_analyst::get_path_info(&state.project_dir).unwrap_or_default();
        let message = state.get_input_value_from_focused(FocusedBlock::Message);
        let mission_data = MissionData {
            project_dir: state.project_dir.clone(),
            message: message.clone(),
            project_files: code_analyst::project_files::get_project_files(
                LocalPath(state.project_dir.clone()),
                &pathinfo.code_language,
                &pathinfo.framework,
            ),
            code_language: pathinfo.code_language,
            framework: pathinfo.framework,
        };
        if &mission_data.message == "" {
            state.set_header_status(HeaderStatus::ErrorMessage(String::from(
                "Message cannot be empty",
            )));
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        let res_ctx = match self.generate_context_files(state, &mission_data).await {
            Ok(res_ctx) => res_ctx,
            Err(err) => {
                state.set_header_status(HeaderStatus::ErrorMessage(err.message));
                return Ok(ShortcutHandlerResponse::StopPropagation);
            }
        };
        let res_exec = match self.execute_mission(state, mission_data, res_ctx).await {
            Ok(res_exec) => res_exec,
            Err(err) => {
                state.set_header_status(HeaderStatus::ErrorMessage(err.message));
                return Ok(ShortcutHandlerResponse::StopPropagation);
            }
        };
        let mission_actions = res_exec.original_actions;
        state.set_action_items(mission_actions);
        Ok(ShortcutHandlerResponse::StopPropagation)
    }

    async fn execute_mission(
        &mut self,
        state: &mut AppState,
        mission_data: MissionData,
        res_ctx: CreateMissionResponse,
    ) -> Result<MissionExecution, ApiError> {
        state.set_header_status(HeaderStatus::LoadingMessage(String::from(
            "Executing mission...",
        )));
        let context_files = res_ctx
            .context_files
            .iter()
            .map(|file_path| MissionExecutionContextFile {
                path: file_path.clone(),
                content: self.get_context_file_content(state, file_path),
            })
            .collect::<Vec<MissionExecutionContextFile>>();
        let req_exec = ExecuteMissionRequest {
            mission_id: res_ctx.mission_id.clone(),
            mission_data,
            context_files,
        };
        let res_exec = match code_missions_api::execute_mission(req_exec).await {
            Ok(response) => {
                state.set_header_status(HeaderStatus::Idle);
                response
            }
            Err(err) => {
                return Err(err);
            }
        };
        state.set_header_status(HeaderStatus::Idle);
        Ok(res_exec)
    }

    fn get_context_file_content(&mut self, state: &mut AppState, file_path: &str) -> String {
        let project_dir = state.project_dir.clone();
        let content = std::fs::read_to_string(format!("{}/{}", project_dir, file_path)).unwrap();
        content
    }

    async fn generate_context_files(
        &mut self,
        state: &mut AppState,
        mission_data: &MissionData,
    ) -> Result<CreateMissionResponse, ApiError> {
        state.set_header_status(HeaderStatus::LoadingMessage(String::from(
            "Generating context files...",
        )));
        let req_ctx = CreateMissionRequest {
            mission_data: mission_data.clone(),
        };
        let res_ctx = match code_missions_api::create_mission(req_ctx).await {
            Ok(response) => {
                state.set_header_status(HeaderStatus::Idle);
                state.set_context_items(
                    response
                        .context_files
                        .clone()
                        .iter()
                        .map(|f| (f.as_str(), f.as_str()))
                        .collect::<Vec<(&str, &str)>>(),
                );
                response
            }
            Err(err) => return Err(err),
        };
        Ok(res_ctx)
    }
}

impl AppViewTrait for MissionView {
    fn components(&mut self, state: &mut AppState) -> Result<HashMap<String, Mutex<AppComponent>>> {
        let el_message = MessageInputComponent::new()?;
        let el_header = HeaderComponent::new()?;
        let el_context_files = ContextFilesComponent::new(FocusedBlock::ContextFiles)?;
        let el_actions = ActionsComponent::new(FocusedBlock::Actions)?;
        let el_shortcuts = ShortcutsComponent::new()?;
        let el_action_preview = ActionPreviewComponent::new()?;

        let mut components = HashMap::new();
        components.insert(String::from("message"), el_message.as_mutex());
        components.insert(String::from("header"), el_header.as_mutex());
        components.insert(String::from("context_files"), el_context_files.as_mutex());
        components.insert(String::from("actions"), el_actions.as_mutex());
        components.insert(String::from("shortcuts"), el_shortcuts.as_mutex());

        // all components below should be rendered to the same position
        let content_position = "project_info";
        if state.focused_block == FocusedBlock::Actions {
            components.insert(String::from(content_position), el_action_preview.as_mutex());
        } else {
            let el_project_info = ProjectInfoComponent::new(state.project_dir.clone())?;
            components.insert(String::from(content_position), el_project_info.as_mutex());
        }

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
}
