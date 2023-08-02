use std::{
    cell::{Cell, RefCell},
    default,
};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout},
    widgets::{Block, Borders, ListState, Paragraph},
    Frame,
};

use crate::components::{
    actions::ActionsComponent, context_files::ContextFilesComponent, header::HeaderComponent,
    message_input::MessageInputComponent, project_info::ProjectInfoComponent,
    shortcuts::ShortcutsComponent, DrawableComponent,
};

pub struct App {
    focused_block: FocusedBlock,
    pub el_message: MessageInputComponent,
    el_header: HeaderComponent,
    pub el_context_files: ContextFilesComponent,
    el_actions: ActionsComponent,
    el_shortcuts: ShortcutsComponent,
    el_project_info: ProjectInfoComponent,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub enum FocusedBlock {
    #[default]
    Home,
    Message,
    ContextFiles,
    Actions,
}

impl App {
    pub fn new(project_dir: String) -> Result<Self> {
        let focused_block = FocusedBlock::default();
        let el_message = MessageInputComponent::new()?;
        let el_header = HeaderComponent::new("Mission Control".to_string())?;
        let el_context_files = ContextFilesComponent::new()?;
        let el_actions = ActionsComponent::new()?;
        let el_shortcuts = ShortcutsComponent::from_focused_block(focused_block.clone())?;
        let el_project_info = ProjectInfoComponent::new(project_dir)?;
        Ok(Self {
            focused_block,
            el_message,
            el_header,
            el_context_files,
            el_actions,
            el_shortcuts,
            el_project_info,
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
                Constraint::Min(5),
                Constraint::Length(self.el_context_files.height()),
                Constraint::Length(5),
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
        self.el_project_info.draw(frame, right_rect)?;

        Ok(())
    }

    // true = should exit
    pub fn handle_events(&mut self) -> Result<bool> {
        self.el_shortcuts.handle_events(
            &mut self.el_header,
            &mut self.el_message,
            &mut self.el_context_files,
        )
    }
}
