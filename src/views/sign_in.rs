use std::{collections::HashMap, rc::Rc, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use lilicore::auth::{auth_login, AuthLoginRequest};
use ratatui::{prelude::*, Frame};

use crate::{
    app::{AppScreen, AppState, FocusedBlock},
    components::{
        button::ButtonComponent,
        header::{HeaderComponent, HeaderStatus},
        shortcuts::ShortcutsComponent,
        text_input::TextInputComponent,
        AppComponent,
    },
    redraw_app,
    shortcuts::{handle_text_input_event, ShortcutHandlerResponse},
};

use super::AppViewTrait;

pub struct SignInView;

impl SignInView {
    pub fn new() -> Self {
        Self {}
    }

    async fn login(&mut self, state: &mut AppState) -> Result<ShortcutHandlerResponse> {
        state.set_header_status(HeaderStatus::Loading);
        let username = state.get_input_value_from_focused(FocusedBlock::UsernameInput);
        let password = state.get_input_value_from_focused(FocusedBlock::PasswordInput);
        if &username == "" {
            state.set_focused_block(FocusedBlock::UsernameInput);
            state.set_header_status(HeaderStatus::ErrorMessage(String::from(
                "Username and password are required",
            )));
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        if &password == "" {
            state.set_focused_block(FocusedBlock::PasswordInput);
            state.set_header_status(HeaderStatus::ErrorMessage(String::from(
                "Username and password are required",
            )));
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        let request = AuthLoginRequest { username, password };
        let response = match auth_login(request).await {
            Ok(response) => response,
            Err(err) => {
                state.set_header_status(HeaderStatus::ErrorMessage(format!(
                    "{:?}",
                    err.error_description
                )));
                return Ok(ShortcutHandlerResponse::StopPropagation);
            }
        };
        // state.set_header_status(HeaderStatus::ErrorMessage(String::from(
        //     "error message goes here",
        // )));
        state.signed_in = true;
        state.set_header_status(HeaderStatus::Idle);
        let decoded = match response.decode_access_token() {
            Ok(decoded) => decoded,
            Err(err) => {
                state.set_header_status(HeaderStatus::ErrorMessage(format!("{:?}", err)));
                return Ok(ShortcutHandlerResponse::StopPropagation);
            }
        };
        let name = decoded.given_name;
        state.set_user_name(name);
        state.set_screen(AppScreen::Mission);
        state.set_focused_block(FocusedBlock::Home);
        return Ok(ShortcutHandlerResponse::StopPropagation);
    }

    pub async fn handle_events(
        &mut self,
        state: &mut AppState,
        key: &KeyEvent,
    ) -> Result<ShortcutHandlerResponse> {
        let unique_name = &TextInputComponent::unique_name_from_focused_block(&state.focused_block);

        match key.code {
            KeyCode::Esc => {
                state.set_screen(AppScreen::Mission);
                state.set_focused_block(FocusedBlock::Home);
                return Ok(ShortcutHandlerResponse::StopPropagation);
            }
            _ => {}
        }

        let focused_block = &state.focused_block.clone();

        match focused_block {
            FocusedBlock::UsernameInput => {
                match key.code {
                    KeyCode::Tab | KeyCode::Enter => {
                        state.set_focused_block(FocusedBlock::PasswordInput);
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                    _ => {}
                }
                return handle_text_input_event(state, key, focused_block);
            }
            FocusedBlock::PasswordInput => {
                match key.code {
                    KeyCode::Enter => {
                        return self.login(state).await;
                    }
                    KeyCode::Tab => {
                        state.set_focused_block(FocusedBlock::SignInButton);
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                    KeyCode::BackTab => {
                        state.set_focused_block(FocusedBlock::UsernameInput);
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                    _ => {}
                }
                return handle_text_input_event(state, key, focused_block);
            }
            FocusedBlock::SignInButton => match key.code {
                KeyCode::Enter => {
                    // todo: clear form
                    // todo: send login
                    return self.login(state).await;
                }
                KeyCode::BackTab => {
                    state.set_focused_block(FocusedBlock::PasswordInput);
                    return Ok(ShortcutHandlerResponse::StopPropagation);
                }
                _ => {}
            },
            _ => {}
        };

        Ok(ShortcutHandlerResponse::Continue)
    }
}

impl AppViewTrait for SignInView {
    fn components(&mut self, state: &mut AppState) -> Result<HashMap<String, Mutex<AppComponent>>> {
        let username_input = TextInputComponent::new("username", FocusedBlock::UsernameInput)?;
        let password_input =
            TextInputComponent::new("password", FocusedBlock::PasswordInput)?.is_password();
        let signin_button = ButtonComponent::new("Sign In", FocusedBlock::SignInButton)?;
        let shortcuts = ShortcutsComponent::new()?;
        let header = HeaderComponent::new()?;

        let mut components = HashMap::new();
        components.insert(String::from("username_input"), username_input.as_mutex());
        components.insert(String::from("password_input"), password_input.as_mutex());
        components.insert(String::from("signin_button"), signin_button.as_mutex());
        components.insert(String::from("shortcuts"), shortcuts.as_mutex());
        components.insert(String::from("header"), header.as_mutex());

        Ok(components)
    }

    fn positions<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        state: &mut AppState,
    ) -> Result<HashMap<String, Rect>> {
        let [top_rect, _, main_rect, _, bottom_rect] = *Layout::default()
          .direction(Direction::Vertical)
          .constraints(
              [
                  Constraint::Length(1),
                  Constraint::Length(2),
                  Constraint::Length(9),
                  Constraint::Min(1),
                  Constraint::Length(1),
              ]
              .as_ref(),
          )
          .split(frame.size())
          else {
              return Ok(HashMap::new());
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
              return Ok(HashMap::new());
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
              return Ok(HashMap::new());
          };

        let mut positions = HashMap::new();
        positions.insert(String::from("username_input"), username_rect);
        positions.insert(String::from("password_input"), password_rect);
        positions.insert(String::from("signin_button"), button_rect);
        positions.insert(String::from("shortcuts"), bottom_rect);
        positions.insert(String::from("header"), top_rect);

        Ok(positions)
    }
}
