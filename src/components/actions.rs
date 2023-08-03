use std::sync::Mutex;

use anyhow::Result;
use ratatui::{
    prelude::{Backend, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::AppState;

use super::{AppComponent, DrawableComponent};

pub struct ActionsComponent {}

impl ActionsComponent {
    pub fn new() -> Result<Self> {
        Ok(Self {})
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
        let block = Block::default().borders(Borders::ALL).title("Actions");

        frame.render_widget(block, rect);
        Ok(())
    }
}
