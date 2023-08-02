use anyhow::Result;
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::DrawableComponent;

pub struct ShortcutsComponent<'a> {
    pub shortcuts: Vec<(&'a str, &'a str)>,
}

impl<'a> ShortcutsComponent<'a> {
    pub fn new(shortcuts: Vec<(&'a str, &'a str)>) -> Result<Self> {
        Ok(Self { shortcuts })
    }
}

impl<'a> DrawableComponent for ShortcutsComponent<'a> {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        let shortcuts = self
            .shortcuts
            .iter()
            .map(|(key, action)| format!("{}) {}", key, action))
            .collect::<Vec<String>>()
            .join("      ");

        let shortcuts = Paragraph::new(shortcuts.as_str())
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left);
        f.render_widget(shortcuts, rect);
        Ok(())
    }
}
