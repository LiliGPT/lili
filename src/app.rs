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
}

impl AppState {
    pub fn new(project_dir: String) -> Result<Self> {
        Ok(Self {
            project_dir,
            screen: AppScreen::default(),
            focused_block: FocusedBlock::default(),
            signed_in: false,
            input_values: HashMap::new(),
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

    // // true = should exit
    // pub fn handle_events(&mut self) -> Result<bool> {
    //     match self.el_shortcuts.handle_events(
    //         self.state.signed_in.clone(),
    //         self.state.focused_block.clone(),
    //         &mut self.el_header,
    //         &mut self.el_message,
    //         &mut self.el_context_files,
    //         &mut self.el_username_input,
    //         &mut self.el_password_input,
    //     ) {
    //         Ok(ShortcutHandlerResponse::Exit) => Ok(true),
    //         Ok(ShortcutHandlerResponse::Continue) => Ok(false),
    //         Ok(ShortcutHandlerResponse::Login)
    //         | Ok(ShortcutHandlerResponse::FocusSignInUsername) => {
    //             self.state.set_screen(AppScreen::SignIn);
    //             self.state.focused_block = FocusedBlock::UsernameInput;
    //             self.el_message.set_focus(false);
    //             self.el_context_files.set_focus(false);
    //             self.el_username_input.set_focus(true);
    //             self.el_password_input.set_focus(false);
    //             self.el_signin_button.set_focus(false);
    //             Ok(false)
    //         }
    //         Ok(ShortcutHandlerResponse::FocusMessage) => {
    //             self.state.set_screen(AppScreen::Mission);
    //             self.state.focused_block = FocusedBlock::Message;
    //             self.el_message.set_focus(true);
    //             self.el_context_files.set_focus(false);
    //             self.el_username_input.set_focus(false);
    //             self.el_password_input.set_focus(false);
    //             self.el_signin_button.set_focus(false);
    //             Ok(false)
    //         }
    //         Ok(ShortcutHandlerResponse::Mission) => {
    //             self.state.set_screen(AppScreen::Mission);
    //             self.state.focused_block = FocusedBlock::Home;
    //             self.el_message.set_focus(false);
    //             self.el_context_files.set_focus(false);
    //             self.el_username_input.set_focus(false);
    //             self.el_password_input.set_focus(false);
    //             self.el_signin_button.set_focus(false);
    //             Ok(false)
    //         }
    //         Ok(ShortcutHandlerResponse::FocusContext) => {
    //             self.state.set_screen(AppScreen::Mission);
    //             self.state.focused_block = FocusedBlock::ContextFiles;
    //             self.el_message.set_focus(false);
    //             self.el_context_files.set_focus(true);
    //             self.el_username_input.set_focus(false);
    //             self.el_password_input.set_focus(false);
    //             self.el_signin_button.set_focus(false);
    //             Ok(false)
    //         }
    //         Ok(ShortcutHandlerResponse::FocusSignInPassword) => {
    //             self.state.set_screen(AppScreen::SignIn);
    //             self.state.focused_block = FocusedBlock::PasswordInput;
    //             self.el_message.set_focus(false);
    //             self.el_context_files.set_focus(false);
    //             self.el_username_input.set_focus(false);
    //             self.el_password_input.set_focus(true);
    //             self.el_signin_button.set_focus(false);
    //             Ok(false)
    //         }
    //         Ok(ShortcutHandlerResponse::FocusSignInButton) => {
    //             self.state.set_screen(AppScreen::SignIn);
    //             self.state.focused_block = FocusedBlock::SignInButton;
    //             self.el_message.set_focus(false);
    //             self.el_context_files.set_focus(false);
    //             self.el_username_input.set_focus(false);
    //             self.el_password_input.set_focus(false);
    //             self.el_signin_button.set_focus(true);
    //             Ok(false)
    //         }
    //         Err(err) => return Err(err),
    //     }
    // }
}
