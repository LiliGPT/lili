use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::FocusedBlock;

use super::{
    context_files::ContextFilesComponent, message_input::MessageInputComponent, DrawableComponent,
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
                }
                _ => {}
            }
        }
        Ok(false)
    }
}

impl DrawableComponent for ShortcutsComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        let shortcuts = self
            .get_shortcuts()
            .iter()
            .map(|(key, action)| format!("{}) {}", key, action))
            .collect::<Vec<String>>()
            .join("      ");

        let shortcuts = Paragraph::new(shortcuts.as_str())
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left);
        f.render_widget(shortcuts, rect);
        Ok(())
    }
}
