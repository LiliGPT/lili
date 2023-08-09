use std::sync::Mutex;

use anyhow::Result;
use lilicore::code_missions_api::MissionAction;
use ratatui::{
    prelude::{Backend, Rect},
    Frame,
};

use crate::{
    app::{AppState, FocusedBlock},
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

    fn get_content(&self, state: &mut AppState) -> String {
        match state.focused_block.clone() {
            FocusedBlock::Actions => {
                let action = match state.action_items.get_selected_item() {
                    Some(action) => action,
                    None => return String::new(),
                };

                return action.content.clone();
            }
            FocusedBlock::ContextFiles => {
                let context_file = match state.context_items.get_selected_item() {
                    Some(item) => item.0.clone(),
                    None => return String::new(),
                };
                let file_content =
                    std::fs::read_to_string(format!("{}/{}", &state.project_dir, context_file))
                        .unwrap_or_default();
                return file_content;
            }
            _ => return String::new(),
        };
    }
}

impl DrawableComponent for ActionPreviewComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        // let action = match state.action_items.get_selected_item() {
        //     Some(action) => action,
        //     None => return Ok(()),
        // };
        let content = self.get_content(state);
        let selected_title = match &state.focused_block {
            &FocusedBlock::Actions => {
                format!(
                    "Action ( {} )",
                    match state.action_items.get_selected_item() {
                        Some(action) => action.path.clone(),
                        None => String::from(""),
                    }
                )
            }
            &FocusedBlock::ContextFiles => {
                format!(
                    "Context File ( {} )",
                    match state.context_items.get_selected_item() {
                        Some(item) => item.0.clone(),
                        None => String::from(""),
                    }
                )
            }
            _ => String::from("Empty Preview"),
        };

        let block = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title(selected_title);

        let text = ratatui::widgets::Paragraph::new(content).block(block);

        frame.render_widget(text, rect);

        Ok(())
    }
}

// fn action_get_newest_content(action: &MissionAction) -> &str {
//     action.content.split('\n').last().unwrap_or(&action.content)
// }

// fn action_get_original_content(action: &MissionAction, state: &mut AppState) -> String {
//     let path = &action.path;
//     let project_dir = &state.project_dir;
//     let full_path = format!("{}/{}", project_dir, path);
//     let original_content = std::fs::read_to_string(full_path).unwrap_or_default();
//     original_content
// }
