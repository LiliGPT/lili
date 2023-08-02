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
    DrawableComponent,
};

pub struct ShortcutsComponent {
    pub focused_block: FocusedBlock,
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

    // true = should exit
    pub fn handle_events(
        &mut self,
        header: &mut HeaderComponent,
        message: &mut MessageInputComponent,
        context_files: &mut ContextFilesComponent,
    ) -> Result<bool> {
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Esc = key.code {
                self.focused_block = FocusedBlock::Home;
                message.set_focus(false);
                context_files.set_focus(false);
            }

            match self.focused_block {
                FocusedBlock::Message => {}
                _ => {
                    if let KeyCode::Char('c') = key.code {
                        self.focused_block = FocusedBlock::ContextFiles;
                        message.set_focus(false);
                        context_files.set_focus(true);
                        return Ok(false);
                    }
                    if let KeyCode::Char('i') = key.code {
                        self.focused_block = FocusedBlock::Message;
                        message.set_focus(true);
                        context_files.set_focus(false);
                        return Ok(false);
                    }
                    if let KeyCode::Char('q') = key.code {
                        return Ok(true);
                    }
                }
            };

            match self.focused_block {
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
                    // if the char is writtable, call message.append_char
                    if let KeyCode::Char(key) = key.code {
                        if key.is_ascii() && !key.is_control() {
                            message.append_char(key);
                        }
                    }
                    // if is Enter
                    if let KeyCode::Enter = key.code {
                        // send message
                        // clear message
                        // message.clear();
                        if header.get_status() == HeaderStatus::Idle {
                            header.set_status(HeaderStatus::Loading);
                        } else {
                            header.set_status(HeaderStatus::Idle);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(false)
    }
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
