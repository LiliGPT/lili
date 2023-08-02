use anyhow::Result;
use ratatui::{
    prelude::{Backend, Rect},
    Frame,
};

pub mod actions;
pub mod context_files;
pub mod header;
pub mod message_input;
pub mod shortcuts;

pub trait DrawableComponent {
    ///
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()>;
}
