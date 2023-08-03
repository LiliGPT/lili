use anyhow::Result;
use ratatui::{
    prelude::{Backend, Rect},
    Frame,
};

pub mod actions;
pub mod button;
pub mod context_files;
pub mod header;
pub mod message_input;
pub mod project_info;
pub mod shortcuts;
pub mod text_input;

pub trait DrawableComponent {
    ///
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()>;
}

pub trait InputComponent {
    fn set_focus(&mut self, focused: bool);
    fn set_value(&mut self, value: String);
    fn value(&self) -> String;
}
