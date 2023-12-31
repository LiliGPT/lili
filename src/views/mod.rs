use crate::{app::AppState, components::AppComponent, shortcuts::ShortcutHandlerResponse};
use anyhow::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::{Backend, Rect},
    Frame,
};
use std::{any::Any, collections::HashMap, rc::Rc, sync::Mutex};

mod add_context_files;
mod commit_temp_branch;
mod create_temp_branch;
mod mission;
mod sign_in;

pub use add_context_files::*;
pub use commit_temp_branch::*;
pub use create_temp_branch::*;
pub use mission::*;
pub use sign_in::*;

pub trait AppViewTrait {
    fn components(&mut self, state: &mut AppState) -> Result<HashMap<String, Mutex<AppComponent>>>;
    fn positions<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        state: &mut AppState,
    ) -> Result<HashMap<String, Rect>>;
}

pub enum AppView {
    Mission(MissionView),
    SignIn(SignInView),
    CreateTempBranch(CreateTempBranchView),
    CommitTempBranch(CommitTempBranchView),
    AddContextFiles(AddContextFilesView),
}

impl AppView {
    pub fn draw<B: Backend>(&mut self, state: &mut AppState, frame: &mut Frame<B>) -> Result<()> {
        let components = match self {
            AppView::Mission(view) => view.components(state),
            AppView::SignIn(view) => view.components(state),
            AppView::CreateTempBranch(view) => view.components(state),
            AppView::CommitTempBranch(view) => view.components(state),
            AppView::AddContextFiles(view) => view.components(state),
        }?;

        let positions = match self {
            AppView::Mission(view) => view.positions(frame, state),
            AppView::SignIn(view) => view.positions(frame, state),
            AppView::CreateTempBranch(view) => view.positions(frame, state),
            AppView::CommitTempBranch(view) => view.positions(frame, state),
            AppView::AddContextFiles(view) => view.positions(frame, state),
        }?;

        for (name, component) in components {
            let mut component = component.lock().unwrap();
            if let Some(rect) = positions.get(&name) {
                component.draw(state, frame, *rect)?;
            }
        }

        Ok(())
    }

    pub async fn handle_events(
        &mut self,
        state: &mut AppState,
        key: &KeyEvent,
    ) -> Result<ShortcutHandlerResponse> {
        // if event is a key release, ignore it
        if &key.kind == &KeyEventKind::Release {
            return Ok(ShortcutHandlerResponse::Continue);
        }
        return match self {
            AppView::Mission(view) => view.handle_events(state, &key).await,
            AppView::SignIn(view) => view.handle_events(state, &key).await,
            AppView::CreateTempBranch(view) => view.handle_events(state, &key).await,
            AppView::CommitTempBranch(view) => view.handle_events(state, &key).await,
            AppView::AddContextFiles(view) => view.handle_events(state, &key).await,
        };
    }
}
