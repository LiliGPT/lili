use std::cell::{Cell, RefCell};

use anyhow::Result;
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout},
    widgets::{Block, Borders, ListState, Paragraph},
    Frame,
};

use crate::components::{
    actions::ActionsComponent, context_files::ContextFilesComponent, header::HeaderComponent,
    message_input::MessageInputComponent, shortcuts::ShortcutsComponent, DrawableComponent,
};

pub struct App<'a> {
    pub el_message: MessageInputComponent,
    el_header: HeaderComponent,
    pub el_context_files: ContextFilesComponent,
    el_actions: ActionsComponent,
    el_shortcuts: ShortcutsComponent<'a>,
}

#[derive(Default)]
pub struct AppState {
    pub message: String,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self> {
        let el_message = MessageInputComponent::new()?;
        let el_header = HeaderComponent::new("Mission Control".to_string())?;
        let el_context_files = ContextFilesComponent::new()?;
        let el_actions = ActionsComponent::new()?;
        let el_shortcuts = ShortcutsComponent::new(vec![
            ("i", "create mission"),
            ("c", "context"),
            ("r", "reset"),
            ("g", "git"),
            ("s", "settings"),
            ("h", "help"),
        ])?;
        Ok(Self {
            el_message,
            el_header,
            el_context_files,
            el_actions,
            el_shortcuts,
        })
    }

    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) -> Result<()> {
        let [top_rect, main_rect, bottom_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(4),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(frame.size())
            else {
                return Ok(());
            };

        let [left_rect, right_rect] = *Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(0)
            .vertical_margin(0)
            .constraints([Constraint::Ratio(2, 6), Constraint::Ratio(4, 6)].as_ref())
            .split(main_rect)
            else {
                return Ok(());
            };

        let [left_top_rect, left_mid_rect, left_bottom_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(3, 12),
                Constraint::Ratio(4, 12),
                Constraint::Ratio(5, 12),
            ].as_ref())
            .split(left_rect)
            else {
                return Ok(());
            };

        self.el_message.draw(frame, left_top_rect)?;
        self.el_header.draw(frame, top_rect)?;
        self.el_context_files.draw(frame, left_mid_rect)?;
        self.el_actions.draw(frame, left_bottom_rect)?;
        self.el_shortcuts.draw(frame, bottom_rect)?;

        Ok(())
    }
}
