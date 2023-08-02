use anyhow::Result;
use ratatui::{
    prelude::{Backend, Constraint, Rect},
    style::Stylize,
    widgets::{Block, Borders, Wrap},
    Frame,
};

use super::DrawableComponent;

pub struct MessageInputComponent {
    focused: bool,
    message: String,
}

impl MessageInputComponent {
    pub fn new() -> Result<Self> {
        Ok(Self {
            focused: false,
            message: String::new(),
        })
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn append_char(&mut self, key: char) {
        self.message.push(key);
    }
}

impl DrawableComponent for MessageInputComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        let mut block = Block::default().borders(Borders::ALL).title("Message");

        let mut message = ratatui::widgets::Paragraph::new(self.message.as_str())
            .alignment(ratatui::prelude::Alignment::Left)
            .wrap(Wrap { trim: true });

        if self.focused {
            block = block
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
        }

        message = message.block(block);

        f.render_widget(message, rect);

        Ok(())
    }
}
