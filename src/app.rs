use std::{
    cell::{Cell, RefCell},
    default,
};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout},
    widgets::{Block, Borders, ListState, Paragraph},
    Frame,
};

use crate::components::{
    actions::ActionsComponent,
    button::ButtonComponent,
    context_files::ContextFilesComponent,
    header::HeaderComponent,
    message_input::MessageInputComponent,
    project_info::ProjectInfoComponent,
    shortcuts::{ShortcutHandlerResponse, ShortcutsComponent},
    text_input::TextInputComponent,
    DrawableComponent, InputComponent,
};

#[derive(Debug, PartialEq, Default, Clone)]
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
}

impl AppState {
    pub fn new(project_dir: String) -> Result<Self> {
        Ok(Self {
            project_dir,
            screen: AppScreen::default(),
            focused_block: FocusedBlock::default(),
            signed_in: false,
        })
    }

    pub fn set_screen(&mut self, screen: AppScreen) {
        self.screen = screen;
    }
}

pub struct App {
    state: &'static mut AppState,
    pub el_message: MessageInputComponent,
    el_header: HeaderComponent,
    pub el_context_files: ContextFilesComponent,
    el_actions: ActionsComponent,
    el_shortcuts: ShortcutsComponent,
    el_project_info: ProjectInfoComponent,
    el_username_input: TextInputComponent,
    el_password_input: TextInputComponent,
    el_signin_button: ButtonComponent,
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
    Actions,
}

impl App {
    pub fn new(state: &'static mut AppState) -> Result<Self> {
        let el_message = MessageInputComponent::new()?;
        let el_header = HeaderComponent::new("Mission Control".to_string())?;
        let el_context_files = ContextFilesComponent::new()?;
        let el_actions = ActionsComponent::new()?;
        let el_shortcuts = ShortcutsComponent::from_focused_block(state.focused_block.clone())?;
        let el_project_info = ProjectInfoComponent::new(state.project_dir.clone())?;
        let el_username_input = TextInputComponent::new("username")?;
        let el_password_input = TextInputComponent::new("password")?;
        let el_signin_button = ButtonComponent::new("Sign In")?;
        Ok(Self {
            state,
            el_message,
            el_header,
            el_context_files,
            el_actions,
            el_shortcuts,
            el_project_info,
            el_username_input,
            el_password_input,
            el_signin_button,
        })
    }

    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) -> Result<()> {
        match self.state.screen {
            AppScreen::Mission => self.draw_mission(frame)?,
            AppScreen::SignIn => self.draw_sign_in(frame)?,
        }
        Ok(())
    }

    pub fn draw_sign_in<B: Backend>(&mut self, frame: &mut Frame<B>) -> Result<()> {
        let [_, main_rect, _] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(4),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(frame.size())
            else {
                return Ok(());
            };

        let [_, mid_rect, _] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ]
                .as_ref(),
            )
            .split(main_rect)
            else {
                return Ok(());
            };

        let [username_rect, password_rect, button_rect, _] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(mid_rect)
            else {
                return Ok(());
            };

        self.el_username_input.draw(frame, username_rect)?;
        self.el_password_input.draw(frame, password_rect)?;
        self.el_signin_button.draw(frame, button_rect)?;

        Ok(())
    }

    pub fn draw_mission<B: Backend>(&mut self, frame: &mut Frame<B>) -> Result<()> {
        let [top_rect, main_rect, bottom_rect] = *Layout::default()
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
                return Ok(());
            };

        let [left_rect, right_rect] = *Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(0)
            .vertical_margin(0)
            .constraints([Constraint::Ratio(2, 6), Constraint::Ratio(4, 6)].as_ref())
            .split(main_rect)
            else {
                return Ok(());
            };

        let [left_top_rect, left_mid_rect, left_bottom_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),
                Constraint::Length(self.el_context_files.height()),
                Constraint::Length(5),
            ].as_ref())
            .split(left_rect)
            else {
                return Ok(());
            };

        self.el_message.draw(frame, left_top_rect)?;
        self.el_header.draw(frame, top_rect)?;
        self.el_context_files.draw(frame, left_mid_rect)?;
        self.el_actions.draw(frame, left_bottom_rect)?;
        self.el_shortcuts.draw(frame, bottom_rect)?;
        self.el_project_info.draw(frame, right_rect)?;

        Ok(())
    }

    // true = should exit
    pub fn handle_events(&mut self) -> Result<bool> {
        match self.el_shortcuts.handle_events(
            self.state.signed_in.clone(),
            self.state.focused_block.clone(),
            &mut self.el_header,
            &mut self.el_message,
            &mut self.el_context_files,
            &mut self.el_username_input,
            &mut self.el_password_input,
        ) {
            Ok(ShortcutHandlerResponse::Exit) => Ok(true),
            Ok(ShortcutHandlerResponse::Continue) => Ok(false),
            Ok(ShortcutHandlerResponse::Login)
            | Ok(ShortcutHandlerResponse::FocusSignInUsername) => {
                self.state.set_screen(AppScreen::SignIn);
                self.state.focused_block = FocusedBlock::UsernameInput;
                self.el_message.set_focus(false);
                self.el_context_files.set_focus(false);
                self.el_username_input.set_focus(true);
                self.el_password_input.set_focus(false);
                self.el_signin_button.set_focus(false);
                Ok(false)
            }
            Ok(ShortcutHandlerResponse::FocusMessage) => {
                self.state.set_screen(AppScreen::Mission);
                self.state.focused_block = FocusedBlock::Message;
                self.el_message.set_focus(true);
                self.el_context_files.set_focus(false);
                self.el_username_input.set_focus(false);
                self.el_password_input.set_focus(false);
                self.el_signin_button.set_focus(false);
                Ok(false)
            }
            Ok(ShortcutHandlerResponse::Mission) => {
                self.state.set_screen(AppScreen::Mission);
                self.state.focused_block = FocusedBlock::Home;
                self.el_message.set_focus(false);
                self.el_context_files.set_focus(false);
                self.el_username_input.set_focus(false);
                self.el_password_input.set_focus(false);
                self.el_signin_button.set_focus(false);
                Ok(false)
            }
            Ok(ShortcutHandlerResponse::FocusContext) => {
                self.state.set_screen(AppScreen::Mission);
                self.state.focused_block = FocusedBlock::ContextFiles;
                self.el_message.set_focus(false);
                self.el_context_files.set_focus(true);
                self.el_username_input.set_focus(false);
                self.el_password_input.set_focus(false);
                self.el_signin_button.set_focus(false);
                Ok(false)
            }
            Ok(ShortcutHandlerResponse::FocusSignInPassword) => {
                self.state.set_screen(AppScreen::SignIn);
                self.state.focused_block = FocusedBlock::PasswordInput;
                self.el_message.set_focus(false);
                self.el_context_files.set_focus(false);
                self.el_username_input.set_focus(false);
                self.el_password_input.set_focus(true);
                self.el_signin_button.set_focus(false);
                Ok(false)
            }
            Ok(ShortcutHandlerResponse::FocusSignInButton) => {
                self.state.set_screen(AppScreen::SignIn);
                self.state.focused_block = FocusedBlock::SignInButton;
                self.el_message.set_focus(false);
                self.el_context_files.set_focus(false);
                self.el_username_input.set_focus(false);
                self.el_password_input.set_focus(false);
                self.el_signin_button.set_focus(true);
                Ok(false)
            }
            Err(err) => return Err(err),
        }
    }
}
