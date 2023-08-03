use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use anyhow::Result;
use crossterm::event::{self, Event};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout},
    Frame,
};

use crate::{
    shortcuts::{handle_global_shortcuts, ShortcutHandlerResponse},
    utils::list::SelectableList,
    views::{AppView, MissionView, SignInView},
};

#[derive(Debug, PartialEq, Default, Clone, Eq, Hash)]
pub enum AppScreen {
    #[default]
    Mission,
    SignIn,
}

pub struct AppState {
    pub project_dir: String,
    pub focused_block: FocusedBlock,
    pub screen: AppScreen,
    pub signed_in: bool,
    pub input_values: HashMap<String, String>,
    pub context_items: SelectableList,
}

impl AppState {
    pub fn new(project_dir: String) -> Result<Self> {
        Ok(Self {
            project_dir,
            screen: AppScreen::default(),
            focused_block: FocusedBlock::default(),
            signed_in: false,
            input_values: HashMap::new(),
            context_items: SelectableList::new(vec![
                ("context 1", "context 1 content"),
                ("context 2", "context 2 content"),
                ("context 3", "context 3 content"),
            ]),
        })
    }

    pub fn set_screen(&mut self, screen: AppScreen) {
        self.screen = screen;
    }

    pub fn set_focused_block(&mut self, focused_block: FocusedBlock) {
        self.focused_block = focused_block;
    }

    pub fn set_input_value(&mut self, name: &str, value: &str) {
        self.input_values
            .insert(name.to_string(), value.to_string());
    }
}

pub struct App {
    state: Mutex<AppState>,
    views: HashMap<AppScreen, Mutex<AppView>>,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub enum FocusedBlock {
    #[default]
    Home,
    Message,
    ContextFiles,
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

    pub fn handle_events(&mut self) -> Result<bool> {
        let state = self.state.get_mut().unwrap();

        if let Event::Key(key) = event::read()? {
            let response = self
                .views
                .get(&state.screen)
                .unwrap()
                .lock()
                .unwrap()
                .handle_events(state, &key)?;

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
