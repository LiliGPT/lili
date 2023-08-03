use anyhow::Result;
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::DrawableComponent;

pub struct ButtonComponent {
    focused: bool,
    label: String,
}

impl ButtonComponent {
    pub fn new(label: &str) -> Result<Self> {
        Ok(Self {
            focused: false,
            label: label.to_string(),
        })
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}

impl DrawableComponent for ButtonComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        let mut block = Block::default().borders(Borders::ALL);

        if self.focused {
            block = block.border_style(Style::default().fg(Color::Cyan));
        }

        let button = Paragraph::new(format!("{}", self.label.as_str()))
            .alignment(Alignment::Center)
            .block(block);

        f.render_widget(button, rect);

        Ok(())
    }
}
