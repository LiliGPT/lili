use std::{collections::HashMap, rc::Rc, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, Frame};

use crate::{
    app::{AppState, FocusedBlock},
    components::{button::ButtonComponent, text_input::TextInputComponent, AppComponent},
    shortcuts::{handle_text_input_event, ShortcutHandlerResponse},
};

use super::AppViewTrait;

pub struct SignInView;

impl SignInView {
    pub fn new() -> Self {
        Self {}
    }
}

impl AppViewTrait for SignInView {
    fn components(&mut self, state: &mut AppState) -> Result<HashMap<String, Mutex<AppComponent>>> {
        let username_input = TextInputComponent::new("username", FocusedBlock::UsernameInput)?;
        let password_input = TextInputComponent::new("password", FocusedBlock::PasswordInput)?;
        let signin_button = ButtonComponent::new("Sign In", FocusedBlock::SignInButton)?;

        let mut components = HashMap::new();
        components.insert(String::from("username_input"), username_input.as_mutex());
        components.insert(String::from("password_input"), password_input.as_mutex());
        components.insert(String::from("signin_button"), signin_button.as_mutex());

        Ok(components)
    }

    fn positions<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        state: &mut AppState,
    ) -> Result<HashMap<String, Rect>> {
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

        Ok(positions)
    }

    fn handle_events(
        &mut self,
        state: &mut AppState,
        key: &KeyEvent,
    ) -> Result<ShortcutHandlerResponse> {
        let unique_name = &TextInputComponent::unique_name_from_focused_block(&state.focused_block);
        match state.focused_block {
            FocusedBlock::UsernameInput => {
                match key.code {
                    KeyCode::Tab | KeyCode::Enter => {
                        state.set_focused_block(FocusedBlock::PasswordInput);
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                    _ => {}
                }
                return handle_text_input_event(state, key, unique_name);
            }
            FocusedBlock::PasswordInput => {
                match key.code {
                    KeyCode::Tab | KeyCode::Enter => {
                        state.set_focused_block(FocusedBlock::SignInButton);
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                    KeyCode::BackTab => {
                        state.set_focused_block(FocusedBlock::UsernameInput);
                        return Ok(ShortcutHandlerResponse::StopPropagation);
                    }
                    _ => {}
                }
                return handle_text_input_event(state, key, unique_name);
            }
            FocusedBlock::SignInButton => match key.code {
                KeyCode::Enter => {
                    // todo: clear form
                    // todo: send login
                    return Ok(ShortcutHandlerResponse::StopPropagation);
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
