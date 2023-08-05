use std::sync::Mutex;

use anyhow::Result;
use ratatui::{
    prelude::{Backend, Constraint, Rect},
    style::Stylize,
    widgets::{Block, Borders, Wrap},
    Frame,
};

use crate::app::{AppState, FocusedBlock};

use super::super::{AppComponent, DrawableComponent, InputComponent};

pub struct MessageInputComponent {
    // focused: bool,
    // value: String,
}

impl MessageInputComponent {
    pub fn new() -> Result<Self> {
        Ok(Self {
            // focused: false,
            // value: String::new(),
        })
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::MessageInput(self))
    }
}

impl DrawableComponent for MessageInputComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let mut block = Block::default().borders(Borders::ALL).title("Message");

        // let value = state
        //     .input_values
        //     .get(&self.unique_name())
        //     .unwrap_or(&String::from(""))
        //     .clone();
        let value = state.get_input_value_from_focused(FocusedBlock::Message);

        let mut message = ratatui::widgets::Paragraph::new(value)
            .alignment(ratatui::prelude::Alignment::Left)
            .wrap(Wrap { trim: true });

        if state.focused_block == FocusedBlock::Message {
            block = block
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
        }

        message = message.block(block);

        frame.render_widget(message, rect);

        Ok(())
    }
}

// impl InputComponent for MessageInputComponent {
//     // fn unique_name(&self) -> String {
//     //     // String::from("message")
//     //     Self::unique_name()
//     // }
//
//     fn set_value(&mut self, value: String) {
//         self.value = value;
//     }
//
//     fn value(&self) -> String {
//         self.value.clone()
//     }
// }
