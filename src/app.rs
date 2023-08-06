use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{self, Event};
use lilicore::{
    auth::{auth_introspect_token, KeycloakDecodedAccessToken},
    code_analyst::{self, project_files::get_project_files},
    code_missions_api::{MissionAction, MissionActionType},
    configjson, git_repo,
    io::LocalPath,
};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout},
    Frame,
};
use strum::Display;

use crate::{
    components::{header::HeaderStatus, text_input::TextInputComponent},
    redraw_app,
    shortcuts::{handle_global_shortcuts, ShortcutHandlerResponse},
    utils::list::SelectableList,
    views::{
        AddContextFilesView, AppView, CommitTempBranchView, CreateTempBranchView, MissionView,
        SignInView,
    },
};

#[derive(Debug, PartialEq, Default, Clone, Eq, Hash)]
pub enum AppScreen {
    #[default]
    Mission,
    SignIn,
    CreateTempBranch,
    CommitTempBranch,
    AddContextFiles,
}

#[derive(Debug, PartialEq, Default, Clone, Display)]
pub enum FocusedBlock {
    #[default]
    Home,
    Message,
    ContextFiles,
    Actions,
    UsernameInput,
    PasswordInput,
    SignInButton,
    CommitMessage,
    SearchContextFileInput,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub project_dir: String,
    pub focused_block: FocusedBlock,
    pub screen: AppScreen,
    pub signed_in: bool,
    pub input_values: HashMap<String, String>,
    pub context_items: SelectableList<(String, String)>,
    pub action_items: SelectableList<MissionAction>,
    pub header_status: HeaderStatus,
    pub user_name: String,
    pub execution_id: Option<String>,
    // pub base_branch_name: String,
}

impl AppState {
    pub async fn new(project_dir: String) -> Result<Self> {
        // let mocked_action_items: Vec<MissionAction> = vec![
        //     MissionAction {
        //         path: String::from("/test1"),
        //         content: String::from("content from test1"),
        //         action_type: MissionActionType::CreateFile,
        //     },
        //     MissionAction {
        //         path: String::from("/test2"),
        //         content: String::from("content from test2"),
        //         action_type: MissionActionType::CreateFile,
        //     },
        //     MissionAction {
        //         path: String::from("/test3"),
        //         content: String::from("updated content from test3"),
        //         action_type: MissionActionType::UpdateFile,
        //     },
        // ];
        // let base_branch_name = git_repo::get_current_branch_name(&project_dir)?;
        let current_branch_name = git_repo::get_current_branch_name(&project_dir)?;
        let screen = if current_branch_name.clone().starts_with("temp-") {
            AppScreen::default()
        } else {
            AppScreen::CreateTempBranch
        };
        let mut signed_in = false;
        let mut user_name = String::from("Guest");
        let access_token = configjson::get("access_token");
        if access_token.clone().is_some() {
            let access_token = access_token.unwrap().clone();
            // validate access token
            let introspected = auth_introspect_token(&access_token).await;
            match introspected {
                Ok(introspected) => {
                    if introspected.active {
                        signed_in = true;
                    }
                }
                Err(_err) => {
                    // user is not signed in
                }
            };

            // get user name from current access token
            if signed_in.clone() {
                let decoded = KeycloakDecodedAccessToken::new(&access_token).ok();
                if decoded.is_some() {
                    let decoded = decoded.unwrap();
                    user_name = decoded.get_user_name().unwrap_or(String::from("Guest"));
                    // signed_in = true;
                }
            }
        }
        Ok(Self {
            project_dir,
            screen,
            focused_block: FocusedBlock::default(),
            signed_in,
            header_status: HeaderStatus::default(),
            input_values: HashMap::new(),
            user_name,
            context_items: SelectableList::new(vec![]),
            action_items: SelectableList::new(vec![]),
            execution_id: None,
            // base_branch_name: current_branch_name,
        })
    }

    pub fn set_screen(&mut self, screen: AppScreen) {
        self.screen = screen;
    }

    pub fn set_focused_block(&mut self, focused_block: FocusedBlock) {
        self.focused_block = focused_block;
    }

    pub fn set_input_value(&mut self, name: &FocusedBlock, value: &str) {
        self.input_values
            .insert(name.to_string(), value.to_string());
    }

    pub fn set_header_status(&mut self, status: HeaderStatus) {
        self.header_status = status;
        redraw_app(self);
    }

    pub fn get_input_value_from_focused(&self, focused_block: FocusedBlock) -> String {
        self.input_values
            .get(&focused_block.to_string())
            .unwrap_or(&String::new())
            .clone()
    }

    pub fn set_user_name(&mut self, name: String) {
        self.user_name = name;
    }

    pub fn set_context_items(&mut self, items: Vec<(&str, &str)>) {
        self.context_items = SelectableList::new(
            items
                .iter()
                .map(|(name, content)| (name.to_string(), content.to_string()))
                .collect(),
        );
    }

    pub fn set_action_items(&mut self, items: Vec<MissionAction>) {
        self.action_items = SelectableList::new(items.clone());
        if items.len() > 0 {
            self.set_focused_block(FocusedBlock::Actions);
            self.action_items.select(Some(0));
        }
    }

    pub fn get_current_execution_id(&self) -> Option<String> {
        self.execution_id.clone()
    }

    pub fn set_current_execution_id(&mut self, execution_id: Option<String>) {
        self.execution_id = execution_id;
    }

    pub fn get_project_files(&mut self) -> Result<Vec<String>> {
        let project_dir = self.project_dir.clone();
        let project_dir_path = LocalPath(self.project_dir.clone());
        let path_info = match code_analyst::get_path_info(&project_dir) {
            Ok(path_info) => path_info,
            Err(err) => {
                anyhow::bail!("Failed to get path info: {:?}", err);
            }
        };
        let code_language = &path_info.code_language;
        let framework = &path_info.framework;
        let project_files = get_project_files(project_dir_path, code_language, framework);
        Ok(project_files)
    }

    pub fn get_base_branch_name(&self) -> Option<String> {
        let key = format!("base_branch_name_{}", self.project_dir);
        configjson::get(&key)
    }

    pub fn set_base_branch_name(&self, base_branch_name: &str) -> Result<()> {
        let key = format!("base_branch_name_{}", self.project_dir);
        match configjson::set(&key, base_branch_name) {
            Ok(_) => Ok(()),
            Err(err) => {
                anyhow::bail!("Failed to set base branch name: {:?}", err);
            }
        }
    }

    pub fn delete_base_branch_name(&self) -> Result<()> {
        let key = format!("base_branch_name_{}", self.project_dir);
        match configjson::delete(&key) {
            Ok(_) => Ok(()),
            Err(err) => {
                anyhow::bail!("Failed to delete base branch name: {:?}", err);
            }
        }
    }
}

pub struct App {
    state: Mutex<AppState>,
    views: HashMap<AppScreen, Mutex<AppView>>,
}

impl App {
    pub fn new(state: Mutex<AppState>) -> Result<Self> {
        let views = {
            let mut views = HashMap::new();

            views.insert(
                AppScreen::Mission,
                Mutex::new(AppView::Mission(MissionView::new())),
            );

            views.insert(
                AppScreen::SignIn,
                Mutex::new(AppView::SignIn(SignInView::new())),
            );

            views.insert(
                AppScreen::CreateTempBranch,
                Mutex::new(AppView::CreateTempBranch(CreateTempBranchView::new())),
            );

            views.insert(
                AppScreen::CommitTempBranch,
                Mutex::new(AppView::CommitTempBranch(CommitTempBranchView::new())),
            );

            views.insert(
                AppScreen::AddContextFiles,
                Mutex::new(AppView::AddContextFiles(AddContextFilesView::new())),
            );

            views
        };
        Ok(Self { state, views })
    }

    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) -> Result<()> {
        let state = self.state.get_mut().unwrap();

        self.views
            .get(&state.screen)
            .unwrap()
            .lock()
            .unwrap()
            .draw(state, frame)?;

        Ok(())
    }

    pub async fn handle_events(&mut self) -> Result<bool> {
        let state = self.state.get_mut().unwrap();

        if let Event::Key(key) = event::read()? {
            let response = self
                .views
                .get(&state.screen)
                .unwrap()
                .lock()
                .unwrap()
                .handle_events(state, &key)
                .await?;

            if response == ShortcutHandlerResponse::StopPropagation {
                return Ok(false);
            }

            return match handle_global_shortcuts(state, &key)? {
                ShortcutHandlerResponse::Continue => Ok(false),
                ShortcutHandlerResponse::Exit => Ok(true),
                _ => Ok(false),
            };
        }

        Ok(false)
    }
}
