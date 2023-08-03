use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::FocusedBlock;

use super::{
    context_files::ContextFilesComponent,
    header::{HeaderComponent, HeaderStatus},
    message_input::MessageInputComponent,
    DrawableComponent, InputComponent,
};

pub struct ShortcutsComponent {
    pub focused_block: FocusedBlock,
}

pub enum ShortcutHandlerResponse {
    Exit,
    Continue,
    Login,
    Mission,
    FocusMessage,
    FocusContext,
    FocusSignInUsername,
    FocusSignInPassword,
    FocusSignInButton,
}

impl ShortcutsComponent {
    pub fn from_focused_block(focused_block: FocusedBlock) -> Result<Self> {
        Ok(Self { focused_block })
    }

    fn get_shortcuts(&self) -> Vec<(&str, &str)> {
        match self.focused_block {
            FocusedBlock::Home => vec![
                ("i", "create mission"),
                ("c", "context"),
                ("r", "reset"),
                ("g", "git"),
                ("s", "settings"),
                ("h", "help"),
            ],
            FocusedBlock::Message => vec![("Esc", "exit"), ("Enter", "send")],
            _ => vec![],
        }
    }

    pub fn handle_events(
        &mut self,
        signed_in: bool,
        focused_block: FocusedBlock,
        header: &mut HeaderComponent,
        message: &mut MessageInputComponent,
        context_files: &mut ContextFilesComponent,
        username: &mut impl InputComponent,
        password: &mut impl InputComponent,
    ) -> Result<ShortcutHandlerResponse> {
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Esc = key.code {
                message.set_focus(false);
                context_files.set_focus(false);
                return Ok(ShortcutHandlerResponse::Mission);
            }

            let input_components = vec![
                FocusedBlock::Message,
                FocusedBlock::UsernameInput,
                FocusedBlock::PasswordInput,
            ];

            let is_input = &input_components.contains(&focused_block);

            if !is_input {
                if let KeyCode::Char('c') = key.code {
                    return Ok(ShortcutHandlerResponse::FocusContext);
                }
                if let KeyCode::Char('i') = key.code {
                    return Ok(ShortcutHandlerResponse::FocusMessage);
                }
                if let KeyCode::Char('q') = key.code {
                    return Ok(ShortcutHandlerResponse::Exit);
                }
                if let KeyCode::Char('l') = key.code {
                    return Ok(ShortcutHandlerResponse::Login);
                }
                if let KeyCode::Char('m') = key.code {
                    return Ok(ShortcutHandlerResponse::Mission);
                }
            }

            match focused_block {
                FocusedBlock::Home => {}
                FocusedBlock::ContextFiles => {
                    if let KeyCode::Up = key.code {
                        context_files.select_previous();
                    }
                    if let KeyCode::Down = key.code {
                        context_files.select_next();
                    }
                }
                FocusedBlock::Message => {
                    // if is Enter
                    if let KeyCode::Enter = key.code {
                        // todo: send message
                        // clear message
                        // message.clear();
                        if header.get_status() == HeaderStatus::Idle {
                            // header.set_status(HeaderStatus::Loading);
                            if !signed_in {
                                return Ok(ShortcutHandlerResponse::Login);
                            }
                        } else {
                            header.set_status(HeaderStatus::Idle);
                        }
                    }
                    return handle_input_events(key, message);
                }
                FocusedBlock::UsernameInput => {
                    let nextkeys = vec![KeyCode::Enter, KeyCode::Tab];
                    if nextkeys.contains(&key.code) {
                        return Ok(ShortcutHandlerResponse::FocusSignInPassword);
                    }
                    return handle_input_events(key, username);
                }
                FocusedBlock::PasswordInput => {
                    let nextkeys = vec![KeyCode::Enter, KeyCode::Tab];
                    if nextkeys.contains(&key.code) {
                        return Ok(ShortcutHandlerResponse::FocusSignInButton);
                    }
                    if key.code == KeyCode::BackTab {
                        return Ok(ShortcutHandlerResponse::FocusSignInUsername);
                    }
                    return handle_input_events(key, password);
                }
                FocusedBlock::SignInButton => {
                    let nextkeys = vec![KeyCode::Enter, KeyCode::Tab];
                    if nextkeys.contains(&key.code) {
                        // todo: submit
                        return Ok(ShortcutHandlerResponse::Login);
                    }
                    if key.code == KeyCode::BackTab {
                        return Ok(ShortcutHandlerResponse::FocusSignInPassword);
                    }
                }
                _ => {}
            }
        }
        Ok(ShortcutHandlerResponse::Continue)
    }
}

fn handle_input_events(
    key: crossterm::event::KeyEvent,
    input: &mut impl InputComponent,
) -> Result<ShortcutHandlerResponse> {
    // if the char is writtable, call message.append_char
    if let KeyCode::Char(key) = key.code {
        if key.is_ascii() && !key.is_control() {
            let new_value = format!("{}{}", input.value(), key);
            input.set_value(new_value);
            return Ok(ShortcutHandlerResponse::Continue);
        }
    }
    if let KeyCode::Backspace = key.code {
        let value = &input.value();
        if value.len() > 0 {
            input.set_value(value[..value.len() - 1].to_string());
            return Ok(ShortcutHandlerResponse::Continue);
        }
    }
    Ok(ShortcutHandlerResponse::Continue)
}

impl DrawableComponent for ShortcutsComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        // let shortcuts = self
        //     .get_shortcuts()
        //     .iter()
        //     .map(|(key, action)| format!("{}) {}", key, action))
        //     .collect::<Vec<String>>()
        //     .join("      ");
        let mut innerp: Vec<Span> = vec![];

        self.get_shortcuts().iter().for_each(|(key, action)| {
            innerp.push(Span::styled(
                format!("{}", key),
                ratatui::style::Style::default().fg(ratatui::style::Color::White),
            ));
            innerp.push(Span::styled(
                format!(" {}", action),
                ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
            ));
            innerp.push(Span::raw("      "));
        });

        let paragraph = Paragraph::new(Line::from(innerp))
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
        Ok(())
    }
}
