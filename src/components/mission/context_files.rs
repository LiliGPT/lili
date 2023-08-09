use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    sync::Mutex,
};

use anyhow::Result;
use ratatui::{
    prelude::{Backend, Rect},
    widgets::{Block, Borders, ListState},
    Frame,
};

use crate::{
    app::{AppState, FocusedBlock},
    utils::list::SelectableList,
};

use super::super::{AppComponent, DrawableComponent};

pub struct ContextFilesComponent {
    focus_name: FocusedBlock,
}

impl ContextFilesComponent {
    pub fn new(focus_name: FocusedBlock) -> Result<Self> {
        // let items = SelectableList::new(vec!["Context 1", "Context 2", "Context 3"]);
        // let state = Cell::new(ListState::default());
        // state.set(ListState::default().with_selected(Some(1)));
        Ok(Self { focus_name })
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::ContextFiles(self))
    }
}

impl DrawableComponent for ContextFilesComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let items = state.context_items.to_items();
        let mut block = Block::default()
            .borders(Borders::TOP)
            .title(format!("Context Files ({})", items.len()));

        let mut list = ratatui::widgets::List::new(items);

        if self.focus_name == state.focused_block {
            block = block
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
            list = list
                .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
                .highlight_symbol("> ");
        }

        list = list.block(block);

        let list_state =
            &mut ListState::default().with_selected(state.context_items.selected_index);

        // f.render_widget(list, rect);
        frame.render_stateful_widget(list, rect, list_state);
        Ok(())
    }
}
