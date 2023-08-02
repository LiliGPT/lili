use anyhow::Result;
use ratatui::{
    prelude::{Backend, Rect},
    widgets::{Block, Borders},
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
}

impl DrawableComponent for MessageInputComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        // let block = Block::default().borders(Borders::ALL).title("Message");
        // f.render_widget(block, rect);

        let mut block = Block::default().borders(Borders::ALL).title("Message");

        let mut message = ratatui::widgets::Paragraph::new(self.message.as_str())
            .alignment(ratatui::prelude::Alignment::Left);

        // let mut input = TextArea::default();

        if self.focused {
            block = block
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
        }

        message = message.block(block);
        f.render_widget(message, rect);

        // input.set_block(block);

        Ok(())
    }
}
