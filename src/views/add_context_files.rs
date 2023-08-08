use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use lilicore::git_repo::get_git_commit_files;
use ratatui::{
    prelude::{Backend, Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    app::{AppScreen, AppState, FocusedBlock},
    components::{
        header::{HeaderComponent, HeaderStatus},
        shortcuts::ShortcutsComponent,
        text_input::TextInputComponent,
        AppComponent,
    },
    shortcuts::{handle_text_input_event, ShortcutHandlerResponse},
    utils::list::SelectableList,
};

use super::AppViewTrait;

#[derive(Debug, Clone)]
pub enum SearchableListType {
    ProjectFiles,
    GitCommits,
}

pub struct AddContextFilesView {
    selected_items: Vec<String>,
    cursor_index: usize,
}

impl AddContextFilesView {
    pub fn new() -> Self {
        Self {
            selected_items: vec![],
            cursor_index: 0,
        }
    }

    fn get_filtered_list(&mut self, state: &mut AppState) -> SelectableList<(String, String)> {
        let query_value = state.get_input_value_from_focused(FocusedBlock::SearchContextFileInput);
        state
            .searchable_list
            .filter_and_collect(|item| item.0.contains(&query_value))
    }

    fn get_selected_context_file_paths(&mut self, state: &mut AppState) -> Vec<String> {
        state
            .context_items
            .items
            .clone()
            .iter()
            .map(|(path, _)| path.clone())
            .collect()
    }

    // fn get_selected_item(&mut self, state: &mut AppState) -> String {
    //     orig_project_files[self.cursor_index.clone()].clone()
    // }

    fn add_picked_item_to_context(&mut self, state: &mut AppState) {
        // todo: get file content (or maybe remove file content and get on the fly)
        // let selected_item = self.get_selected_item(state);
        let mut filtered_list = self.get_filtered_list(state);
        let list_len = filtered_list.len();
        let real_index = if &list_len > &0 {
            Some(self.cursor_index % filtered_list.len())
        } else {
            None
        };
        filtered_list.select(real_index);
        let default_item = (String::new(), String::new());
        let selected_item = filtered_list
            .get_selected_item()
            .unwrap_or(&default_item)
            .clone();
        let is_selected = self.selected_items.contains(&selected_item.0);
        if is_selected {
            return;
        }
        self.selected_items.push(selected_item.0.clone());
        match state.searchable_list_type {
            SearchableListType::ProjectFiles => {
                state
                    .context_items
                    .items
                    .retain(|(p, _)| p != &selected_item.0);
                // self.selected_items.retain(|p| p != &selected_item.0);
                state.context_items.add_item(selected_item);
            }
            SearchableListType::GitCommits => {
                // state.context_items.add_item(selected_item);
                let commit_hash = selected_item.0.split(" ").next().unwrap_or("");
                if commit_hash.is_empty() {
                    return;
                }
                let commit_files = match get_git_commit_files(commit_hash, &state.project_dir) {
                    Ok(commit_files) => commit_files,
                    Err(e) => {
                        state.set_header_status(HeaderStatus::ErrorMessage(e.to_string()));
                        return;
                    }
                };
                let project_files = match state.get_project_files() {
                    Ok(project_files) => project_files,
                    Err(e) => {
                        state.set_header_status(HeaderStatus::ErrorMessage(format!(
                            "unable to get project_files: {}",
                            e.to_string()
                        )));
                        return;
                    }
                };
                for commit_file in commit_files {
                    let path = commit_file.path;
                    // let project_dir = state.project_dir.clone();
                    // let full_path = format!("{}/{}", project_dir, path);
                    // match std::path::Path::new(&full_path).exists() {
                    match project_files.contains(&path) {
                        true => {
                            let item = (path, String::from(""));
                            state.context_items.items.retain(|(p, _)| p != &item.0);
                            state.context_items.add_item(item);
                        }
                        false => {}
                    };
                }
            }
        };
    }

    pub async fn handle_events(
        &mut self,
        state: &mut AppState,
        key: &KeyEvent,
    ) -> Result<ShortcutHandlerResponse> {
        if KeyCode::Up == key.code {
            self.cursor_index = self.cursor_index.saturating_sub(1);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        if KeyCode::Down == key.code {
            self.cursor_index = self.cursor_index.saturating_add(1);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        // add to context files
        if KeyCode::Char(' ') == key.code {
            self.add_picked_item_to_context(state);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        // exit this component and reset
        if KeyCode::Enter == key.code || KeyCode::Esc == key.code {
            self.cursor_index = 0;
            // orig_project_files = state.get_project_files()?;
            state.set_screen(AppScreen::Mission);
            state.set_focused_block(FocusedBlock::ContextFiles);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        self.cursor_index = 0;
        return handle_text_input_event(state, key, &FocusedBlock::SearchContextFileInput);
    }
}

impl AppViewTrait for AddContextFilesView {
    fn components(&mut self, state: &mut AppState) -> Result<HashMap<String, Mutex<AppComponent>>> {
        let el_header = HeaderComponent::new()?;
        let el_shortcuts = ShortcutsComponent::new()?;
        let el_search =
            TextInputComponent::new("add context files", FocusedBlock::SearchContextFileInput)?;

        let mut components = HashMap::new();
        components.insert(String::from("header"), el_header.as_mutex());
        components.insert(String::from("shortcuts"), el_shortcuts.as_mutex());
        components.insert(String::from("search"), el_search.as_mutex());

        Ok(components)
    }

    fn positions<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        state: &mut AppState,
    ) -> Result<HashMap<String, Rect>> {
        let [top_rect, _main_rect, bottom_rect] = *Layout::default()
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(frame.size())
            else {
                return Ok(HashMap::new());
            };

        let [search_rect, list_rect] = *Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
            ])
            .split(_main_rect)
            else {
                return Ok(HashMap::new());
            };

        let positions = vec![
            (String::from("header"), top_rect),
            (String::from("shortcuts"), bottom_rect),
            (String::from("search"), search_rect),
        ];

        let filter_string =
            state.get_input_value_from_focused(FocusedBlock::SearchContextFileInput);

        // let orig_project_files = state.get_project_files()?;
        // let orig_project_files = orig_project_files
        //     .iter()
        //     .filter(|file| file.contains(&filter_string))
        //     .map(|file| file.to_string())
        //     .collect::<Vec<String>>();

        // let mut selection_map = vec![];

        // let project_files = SelectableList::new(orig_project_files.clone());
        // let project_files = self.list.clone();
        let searchable_items = state
            .searchable_list
            .filter_and_collect(|item| item.0.contains(&filter_string));
        // let mut searchable_items = state.searchable_list.items.clone();
        // let context_items = state.context_items.to_items();

        let list_len = searchable_items.len();
        let real_index = if &list_len > &0 {
            Some(self.cursor_index % searchable_items.len())
        } else {
            None
        };

        for (i, item_str) in self.selected_items.iter().enumerate() {
            let mut was_found = false;
            for searchable_item in searchable_items.items.clone() {
                if item_str == &searchable_item.0 {
                    // state.searchable_itemss
                    // selection_map.push(true);
                    was_found = true;
                    break;
                }
            }
            if !was_found {
                // selection_map.push(false);
            }
        }

        let drawable_items = searchable_items
            .to_items()
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_in_selected_items = self
                    .selected_items
                    .contains(&searchable_items.items[i].0.clone());
                let is_in_context = state
                    .context_items
                    .items
                    .iter()
                    .any(|(p, _)| p == &searchable_items.items[i].0.clone());
                if is_in_selected_items || is_in_context {
                    item.clone()
                        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Green))
                } else {
                    item.clone()
                }
            })
            .collect::<Vec<ListItem<'_>>>();

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Project Files")
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));

        let list = ratatui::widgets::List::new(drawable_items)
            .block(block)
            // .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
            .highlight_symbol("> ");

        let list_state = &mut ListState::default().with_selected(real_index);

        frame.render_stateful_widget(list, list_rect, list_state);

        // let lines: Vec<Line> = project_files
        //     .iter()
        //     .map(|file| Line::from(file.to_string()))
        //     .collect();
        // let widgets = Paragraph::new(lines.clone()).block(
        //     Block::default()
        //         .borders(Borders::ALL)
        //         .title("Project Files"),
        // );
        // frame.render_widget(widgets.clone(), list_rect);

        Ok(positions.into_iter().collect())
    }
}
