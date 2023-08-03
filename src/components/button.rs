use std::sync::Mutex;

use anyhow::Result;
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{AppState, FocusedBlock};

use super::{AppComponent, DrawableComponent};

pub struct ButtonComponent {
    focus_name: FocusedBlock,
    label: String,
}

impl ButtonComponent {
    pub fn new(label: &str, focus_name: FocusedBlock) -> Result<Self> {
        Ok(Self {
            // focused: false,
            focus_name,
            label: label.to_string(),
        })
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::Button(self))
    }
}

impl DrawableComponent for ButtonComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let mut block = Block::default().borders(Borders::ALL);

        if state.focused_block == self.focus_name {
            block = block.border_style(Style::default().fg(Color::Cyan));
        }

        let button = Paragraph::new(format!("{}", self.label.as_str()))
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(button, rect);

        Ok(())
    }
}
