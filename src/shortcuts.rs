use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    app::{AppScreen, AppState, FocusedBlock},
    components::header::HeaderStatus,
};

#[derive(Debug, PartialEq, Clone)]
pub enum ShortcutHandlerResponse {
    Exit,
    StopPropagation,
    Continue,
    Mission,
    SignIn,
}

pub fn handle_global_shortcuts(
    state: &mut AppState,
    key: &KeyEvent,
) -> Result<ShortcutHandlerResponse> {
    if let KeyCode::Esc = key.code {
        state.set_focused_block(FocusedBlock::Home);
        state.set_header_status(HeaderStatus::Idle);
        return Ok(ShortcutHandlerResponse::Mission);
    }

    if let KeyCode::Char('q') = key.code {
        return Ok(ShortcutHandlerResponse::Exit);
    }

    if let KeyCode::Char('l') = key.code {
        state.set_screen(AppScreen::SignIn);
        state.set_focused_block(FocusedBlock::UsernameInput);
        state.set_header_status(HeaderStatus::Idle);
        return Ok(ShortcutHandlerResponse::SignIn);
    }

    if let KeyCode::Char('m') = key.code {
        state.set_screen(AppScreen::Mission);
        state.set_focused_block(FocusedBlock::Home);
        state.set_header_status(HeaderStatus::Idle);
        return Ok(ShortcutHandlerResponse::Mission);
    }

    if let KeyCode::Char('.') = key.code {
        let base_branch_name = state.get_base_branch_name();
        if base_branch_name.is_none() {
            // state.set_header_status(HeaderStatus::ErrorMessage(
            //     "Could not find base branch name".to_string(),
            // ));
            state.set_screen(AppScreen::CreateTempBranch);
            state.set_focused_block(FocusedBlock::Home);
            state.set_header_status(HeaderStatus::Idle);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        state.set_screen(AppScreen::CommitTempBranch);
        state.set_focused_block(FocusedBlock::CommitMessage);
        state.set_header_status(HeaderStatus::Idle);
        return Ok(ShortcutHandlerResponse::StopPropagation);
    }

    Ok(ShortcutHandlerResponse::Continue)
}

pub fn handle_text_input_event(
    state: &mut AppState,
    key: &KeyEvent,
    focus_name: &FocusedBlock,
) -> Result<ShortcutHandlerResponse> {
    let current_value = state.get_input_value_from_focused(focus_name.clone());

    if let KeyCode::Char(key) = key.code {
        if key.is_ascii() && !key.is_control() {
            let new_value = format!("{}{}", &current_value, key);
            state.set_input_value(focus_name, &new_value);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
    }
    if let KeyCode::Backspace = key.code {
        if current_value.len() > 0 {
            let new_value = current_value[..current_value.len() - 1].to_string();
            state.set_input_value(focus_name, &new_value);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
    }
    Ok(ShortcutHandlerResponse::Continue)
}
