use anyhow::Result;
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::DrawableComponent;

pub struct HeaderComponent {
    pub project_name: String,
}

impl HeaderComponent {
    pub fn new(project_name: String) -> Result<Self> {
        Ok(Self { project_name })
    }
}

impl DrawableComponent for HeaderComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        let header = Paragraph::new(self.project_name.as_str())
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left);
        f.render_widget(header, rect);
        Ok(())
    }
}
