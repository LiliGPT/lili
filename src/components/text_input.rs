use std::sync::Mutex;

use anyhow::Result;
use ratatui::{prelude::Rect, Frame};

use crate::app::{AppState, FocusedBlock};

use super::{AppComponent, DrawableComponent, InputComponent};

pub struct TextInputComponent {
    // focused: bool,
    focus_name: FocusedBlock,
    value: String,
    label: String,
}

impl TextInputComponent {
    pub fn new(label: &str, focus_name: FocusedBlock) -> Result<Self> {
        Ok(Self {
            // focused: false,
            focus_name,
            value: String::new(),
            label: label.to_string(),
        })
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::TextInput(self))
    }

    pub fn unique_name_from_focused_block(focused_block: &FocusedBlock) -> String {
        String::from(format!("TextInput_{}", focused_block.clone() as u8))
    }
}

impl DrawableComponent for TextInputComponent {
    fn draw<B: ratatui::prelude::Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> anyhow::Result<()> {
        let mut block = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title(self.label.clone());

        let value = state
            .input_values
            .get(&self.unique_name())
            .unwrap_or(&String::from(""))
            .clone();

        let mut message = ratatui::widgets::Paragraph::new(value)
            .alignment(ratatui::prelude::Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });

        if state.focused_block == self.focus_name {
            block = block
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
        }

        message = message.block(block);

        frame.render_widget(message, rect);

        Ok(())
    }
}

impl InputComponent for TextInputComponent {
    fn unique_name(&self) -> String {
        Self::unique_name_from_focused_block(&self.focus_name)
    }

    fn set_value(&mut self, value: String) {
        self.value = value;
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}
