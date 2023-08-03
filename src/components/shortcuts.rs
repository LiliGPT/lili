use std::sync::Mutex;

use anyhow::Result;
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{AppState, FocusedBlock};

use super::{AppComponent, DrawableComponent};

pub struct ShortcutsComponent {
    pub focused_block: FocusedBlock,
}

impl ShortcutsComponent {
    pub fn from_focused_block(focused_block: FocusedBlock) -> Result<Self> {
        Ok(Self { focused_block })
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::Shortcuts(self))
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
}

impl DrawableComponent for ShortcutsComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
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
        frame.render_widget(paragraph, rect);
        Ok(())
    }
}
