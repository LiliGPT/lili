use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use lilicore::{
    code_analyst,
    code_missions_api::{
        self, set_approved, set_fail, ApiError, CodeMissionStatus, CreateMissionRequest,
        CreateMissionResponse, ExecuteMissionRequest, MissionData, MissionExecution,
        MissionExecutionContextFile, MissionExecutionStatus, SetApprovedRequest, SetFailRequest,
    },
    coder,
    git_repo::{
        get_current_branch_name, get_last_commit_message, git_add_temporary_commit,
        git_undo_last_commit,
    },
    io::LocalPath,
};
use ratatui::{prelude::*, Frame};

use crate::{
    app::{AppScreen, AppState, FocusedBlock},
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
                    let should_generate_context = match state.context_items.items.len() {
                        0 => true,
                        _ => false,
                    };
                    return self.send_message(state, should_generate_context).await;
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
                KeyCode::Char('d') => {
                    state.context_items.remove_selected_item();
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                KeyCode::Char('p') => {
                    state.set_screen(AppScreen::AddContextFiles);
                    state.set_focused_block(FocusedBlock::SearchContextFileInput);
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                KeyCode::Char('x') => {
                    state.set_context_items(vec![]);
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                KeyCode::Char('t') => {
                    let current_context_items = state.context_items.items.clone();
                    let action_context_items = state
                        .action_items
                        .items
                        .clone()
                        .iter()
                        .map(|mission| (mission.path.to_string(), mission.content.to_string()))
                        .collect::<Vec<(String, String)>>();
                    let merged = current_context_items
                        .iter()
                        .chain(action_context_items.iter())
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<(&str, &str)>>();
                    state.set_context_items(merged);
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                KeyCode::Char('o') => {
                    let current_context = state.context_items.get_selected_item();
                    let file_path = match current_context {
                        Some(file_path) => file_path.0.clone(),
                        None => {
                            return Ok(ShortcutHandlerResponse::StopPropagation);
                        }
                    };
                    coder::open_file_in_editor(&state.project_dir, &file_path).ok();
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
                KeyCode::Char('y') => {
                    // approve and run
                    match _approve_and_run(state).await {
                        Ok(_) => {
                            state.set_screen(AppScreen::Mission);
                            state.set_focused_block(FocusedBlock::Home);
                            state.set_input_value(&FocusedBlock::Message, "");
                            state.set_current_execution_id(None);
                            // _replace_context_files_with_actions(state);
                            // state.set_context_items(vec![]);
                            // state.set_action_items(vec![]);
                            state.set_header_status(HeaderStatus::SuccessMessage(String::from(
                                "Mission executed successfully",
                            )));
                            return Ok(ShortcutHandlerResponse::StopPropagation);
                        }
                        Err(err) => {
                            state.set_header_status(HeaderStatus::ErrorMessage(err.to_string()));
                            return Ok(ShortcutHandlerResponse::StopPropagation);
                        }
                    }
                }
                KeyCode::Char('x') => {
                    match state.set_execution_fail().await {
                        Ok(_) => {}
                        Err(err) => {
                            state.set_header_status(HeaderStatus::ErrorMessage(err.to_string()));
                            return Ok(ShortcutHandlerResponse::StopPropagation);
                        }
                    };
                    state.set_screen(AppScreen::Mission);
                    state.set_focused_block(FocusedBlock::Home);
                    state.set_input_value(&FocusedBlock::Message, "");
                    state.set_action_items(vec![]);
                    state.set_header_status(HeaderStatus::Idle);
                    state.set_current_execution_id(None);
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                KeyCode::Char('o') => {
                    let current_action = state.action_items.get_selected_item();
                    let file_path = match current_action {
                        Some(action) => action.path.clone(),
                        None => {
                            return Ok(ShortcutHandlerResponse::StopPropagation);
                        }
                    };
                    match coder::open_file_in_editor(&state.project_dir, &file_path) {
                        Ok(_) => {}
                        Err(err) => {
                            state.set_header_status(HeaderStatus::ErrorMessage(err.to_string()));
                            return Ok(ShortcutHandlerResponse::StopPropagation);
                        }
                    };
                }
                KeyCode::Char(' ') => {
                    let current_action = state.action_items.get_selected_item();
                    let file_path = match current_action {
                        Some(action) => action.path.clone(),
                        None => {
                            return Ok(ShortcutHandlerResponse::StopPropagation);
                        }
                    };
                    state.context_items.add_item((file_path, String::from("")));
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
            KeyCode::Char('u') => {
                let current_branch = match get_last_commit_message(&state.project_dir) {
                    Ok(current_branch) => current_branch,
                    Err(_) => String::from(""),
                };
                if !current_branch.contains("execution-") {
                    state.set_header_status(HeaderStatus::ErrorMessage(String::from(
                        "last commit is not an execution",
                    )));
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                match state.set_execution_fail().await {
                    Ok(_) => {}
                    Err(err) => {
                        state.set_header_status(HeaderStatus::ErrorMessage(err.to_string()));
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                };
                match git_undo_last_commit(&state.project_dir) {
                    Ok(_) => {
                        state.set_header_status(HeaderStatus::SuccessMessage(String::from(
                            "Last commit undone",
                        )));
                    }
                    Err(err) => {
                        state.set_header_status(HeaderStatus::ErrorMessage(err.to_string()));
                    }
                }
                Ok(ShortcutHandlerResponse::StopPropagation)
            }
            _ => Ok(ShortcutHandlerResponse::Continue),
        }
    }

    async fn send_message(
        &mut self,
        state: &mut AppState,
        generate_context: bool,
    ) -> Result<ShortcutHandlerResponse> {
        state.set_header_status(HeaderStatus::LoadingMessage(String::from(
            "Updating previous execution...",
        )));
        state.set_execution_fail().await.ok();
        state.set_header_status(HeaderStatus::LoadingMessage(String::from(
            "Preparing execution...",
        )));
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
        let res_ctx = match generate_context {
            true => match self.generate_context_files(state, &mission_data).await {
                Ok(res_ctx) => res_ctx,
                Err(err) => {
                    state.set_header_status(HeaderStatus::ErrorMessage(err.message));
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
            },
            false => CreateMissionResponse {
                mission_id: String::from(""), // todo: does this throws an error? should be optional?
                context_files: state
                    .context_items
                    .items
                    .iter()
                    .map(|(k, _)| k.clone())
                    .collect(),
                mission_status: CodeMissionStatus::Created,
            },
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
                state.set_current_execution_id(Some(response.execution_id.clone()));
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

async fn _approve_and_run(state: &mut AppState) -> Result<()> {
    let execution_id = match state.get_current_execution_id() {
        Some(execution_id) => execution_id,
        None => {
            anyhow::bail!("No execution id found");
        }
    };
    let req_approved = SetApprovedRequest {
        execution_id: execution_id.clone(),
    };
    match set_approved(req_approved).await {
        Ok(_) => {}
        Err(err) => {
            anyhow::bail!(err.message);
        }
    };
    coder::run_actions(&state.project_dir, &state.action_items.items.as_ref())?;
    git_add_temporary_commit(&state.project_dir, Some(execution_id.clone()))?;
    Ok(())
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
        match state.focused_block {
            FocusedBlock::Actions | FocusedBlock::ContextFiles => {
                components.insert(String::from(content_position), el_action_preview.as_mutex());
            }
            _ => {
                let el_project_info = ProjectInfoComponent::new(state.project_dir.clone())?;
                components.insert(String::from(content_position), el_project_info.as_mutex());
            }
        };

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
