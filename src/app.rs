use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{self, Event};
use lilicore::code_missions_api::{MissionAction, MissionActionType};
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
    views::{AppView, CreateTempBranchView, MissionView, SignInView},
};

#[derive(Debug, PartialEq, Default, Clone, Eq, Hash)]
pub enum AppScreen {
    Mission,
    SignIn,
    #[default]
    CreateTempBranch,
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
}

impl AppState {
    pub fn new(project_dir: String) -> Result<Self> {
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
        Ok(Self {
            project_dir,
            screen: AppScreen::default(),
            focused_block: FocusedBlock::default(),
            signed_in: false,
            header_status: HeaderStatus::default(),
            input_values: HashMap::new(),
            user_name: String::from("Guest"),
            context_items: SelectableList::new(vec![]),
            action_items: SelectableList::new(vec![]),
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
}

pub struct App {
    state: Mutex<AppState>,
    views: HashMap<AppScreen, Mutex<AppView>>,
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
