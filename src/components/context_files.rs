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

use crate::{app::AppState, utils::list::BatchList};

use super::{AppComponent, DrawableComponent};

pub struct ContextFilesComponent {
    focused: bool,
    items: BatchList,
    state: Cell<ListState>,
}

impl ContextFilesComponent {
    pub fn new() -> Result<Self> {
        let items = BatchList::new(vec!["Context 1", "Context 2", "Context 3"]);
        let state = Cell::new(ListState::default());
        state.set(ListState::default().with_selected(Some(1)));
        let focused = false;
        Ok(Self {
            items,
            state,
            focused,
        })
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::ContextFiles(self))
    }

    pub fn select_next(&mut self) {
        if !self.focused {
            return;
        }
        let state = self.state.get_mut();
        let mut idx = state.selected().unwrap_or(0);
        let len = self.items.to_str().len();
        idx = if idx >= len - 1 { 0 } else { idx + 1 };
        state.select(Some(idx));
    }

    pub fn select_previous(&mut self) {
        if !self.focused {
            return;
        }
        let state = self.state.get_mut();
        let mut idx = state.selected().unwrap_or(0);
        let len = self.items.to_str().len();
        idx = if idx == 0 { len - 1 } else { idx - 1 };
        state.select(Some(idx));
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn height(&mut self) -> u16 {
        self.items.len() + 2
    }
}

impl DrawableComponent for ContextFilesComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let items = self.items.to_items();

        let mut block = Block::default()
            .borders(Borders::ALL)
            .title("Context Files");

        let mut list = ratatui::widgets::List::new(items);

        if self.focused {
            block = block
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
            list = list
                .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
                .highlight_symbol("> ");
        }

        list = list.block(block);

        // f.render_widget(list, rect);
        frame.render_stateful_widget(list, rect, self.state.get_mut());
        Ok(())
    }
}
