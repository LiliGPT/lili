use anyhow::Result;

use super::{DrawableComponent, InputComponent};

pub struct TextInputComponent {
    focused: bool,
    value: String,
    label: String,
}

impl TextInputComponent {
    pub fn new(label: &str) -> Result<Self> {
        Ok(Self {
            focused: false,
            value: String::new(),
            label: label.to_string(),
        })
    }
}

impl DrawableComponent for TextInputComponent {
    fn draw<B: ratatui::prelude::Backend>(
        &mut self,
        f: &mut ratatui::Frame<B>,
        rect: ratatui::prelude::Rect,
    ) -> anyhow::Result<()> {
        let mut block = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title(self.label.clone());

        let mut message = ratatui::widgets::Paragraph::new(self.value.as_str())
            .alignment(ratatui::prelude::Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });

        if self.focused {
            block = block
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
        }

        message = message.block(block);

        f.render_widget(message, rect);

        Ok(())
    }
}

impl InputComponent for TextInputComponent {
    fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn set_value(&mut self, value: String) {
        self.value = value;
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}
