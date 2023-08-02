use anyhow::Result;
use ratatui::{
    prelude::{Backend, Rect},
    widgets::{Block, Borders},
    Frame,
};

use super::DrawableComponent;

pub struct ActionsComponent {}

impl ActionsComponent {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl DrawableComponent for ActionsComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        let block = Block::default().borders(Borders::ALL).title("Actions");

        f.render_widget(block, rect);
        Ok(())
    }
}
