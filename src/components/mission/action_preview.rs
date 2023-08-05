use std::sync::Mutex;

use anyhow::Result;
use lilicore::code_missions_api::MissionAction;
use ratatui::{
    prelude::{Backend, Rect},
    Frame,
};

use crate::{
    app::AppState,
    components::{AppComponent, DrawableComponent},
};

pub struct ActionPreviewComponent;

impl ActionPreviewComponent {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::ActionPreview(self))
    }
}

impl DrawableComponent for ActionPreviewComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let action = match state.action_items.get_selected_item() {
            Some(action) => action,
            None => return Ok(()),
        };

        let block = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title("Action Preview");

        let text = ratatui::widgets::Paragraph::new(action.content.clone()).block(block);

        frame.render_widget(text, rect);

        Ok(())
    }
}

fn action_get_newest_content(action: &MissionAction) -> &str {
    action.content.split('\n').last().unwrap_or(&action.content)
}

// fn action_get_original_content(action: &MissionAction, state: &mut AppState) -> String {
//     let path = &action.path;
//     let project_dir = &state.project_dir;
//     let full_path = format!("{}/{}", project_dir, path);
//     let original_content = std::fs::read_to_string(full_path).unwrap_or_default();
//     original_content
// }
