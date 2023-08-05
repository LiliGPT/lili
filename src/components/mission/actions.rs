use std::sync::Mutex;

use anyhow::Result;
use ratatui::{
    prelude::{Backend, Rect},
    widgets::{Block, Borders, ListState},
    Frame,
};

use crate::app::{AppState, FocusedBlock};

use super::super::{AppComponent, DrawableComponent};

pub struct ActionsComponent {
    focus_name: FocusedBlock,
}

impl ActionsComponent {
    pub fn new(focus_name: FocusedBlock) -> Result<Self> {
        Ok(Self { focus_name })
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::Actions(self))
    }
}

impl DrawableComponent for ActionsComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let items = state.action_items.to_items();

        let mut block = Block::default().borders(Borders::ALL).title("Actions");

        let mut list = ratatui::widgets::List::new(items);

        if self.focus_name == state.focused_block {
            block = block
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
            list = list
                .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
                .highlight_symbol("> ");
        }

        list = list.block(block);
        let list_state = &mut ListState::default().with_selected(state.action_items.selected_index);
        frame.render_stateful_widget(list, rect, list_state);
        // frame.render_widget(list, rect);
        Ok(())
    }
}
