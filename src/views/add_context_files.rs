use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Backend, Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, ListState, Paragraph},
    Frame,
};

use crate::{
    app::{AppScreen, AppState, FocusedBlock},
    components::{
        header::HeaderComponent, shortcuts::ShortcutsComponent, text_input::TextInputComponent,
        AppComponent,
    },
    shortcuts::{handle_text_input_event, ShortcutHandlerResponse},
    utils::list::SelectableList,
};

use super::AppViewTrait;

pub struct AddContextFilesView {
    selected_index: usize,
    list_items: Vec<String>,
}

impl AddContextFilesView {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            list_items: vec![],
        }
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

    fn get_selected_item(&mut self, state: &mut AppState) -> String {
        self.list_items[self.selected_index.clone()].clone()
    }

    fn pick_current_item(&mut self, state: &mut AppState) {
        // todo: get file content (or maybe remove file content and get on the fly)
        let selected_item = self.get_selected_item(state);
        state
            .context_items
            .items
            .retain(|(p, _)| p != &selected_item.clone());
        state
            .context_items
            .add_item((selected_item.to_string(), "".to_string()));
    }

    pub async fn handle_events(
        &mut self,
        state: &mut AppState,
        key: &KeyEvent,
    ) -> Result<ShortcutHandlerResponse> {
        if KeyCode::Up == key.code {
            self.selected_index = self.selected_index.saturating_sub(1);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        if KeyCode::Down == key.code {
            self.selected_index = self.selected_index.saturating_add(1);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        // add to context files
        if KeyCode::Char(' ') == key.code {
            self.pick_current_item(state);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        // exit this component and reset
        if KeyCode::Enter == key.code || KeyCode::Esc == key.code {
            self.selected_index = 0;
            self.list_items = state.get_project_files()?;
            state.set_screen(AppScreen::Mission);
            state.set_focused_block(FocusedBlock::ContextFiles);
            return Ok(ShortcutHandlerResponse::StopPropagation);
        }
        self.selected_index = 0;
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

        let orig_project_files = state.get_project_files()?;
        let orig_project_files = orig_project_files
            .iter()
            .filter(|file| file.contains(&filter_string))
            .map(|file| file.to_string())
            .collect::<Vec<String>>();
        self.list_items = orig_project_files;

        let mut selection_map = vec![];

        let project_files = SelectableList::new(self.list_items.clone());
        let mut project_files = project_files.to_items();
        // let context_items = state.context_items.to_items();

        let list_len = project_files.len();
        let real_index = if &list_len > &0 {
            self.selected_index % project_files.clone().len()
        } else {
            0
        };

        for (i, item) in self.list_items.clone().iter().enumerate() {
            let mut was_found = false;
            for context_item in state.context_items.items.clone() {
                if item == &context_item.0 {
                    // state.context_items
                    selection_map.push(true);
                    was_found = true;
                    break;
                }
            }
            if !was_found {
                selection_map.push(false);
            }
        }

        for (i, projectfile) in project_files.clone().iter().enumerate() {
            if selection_map[i] {
                project_files[i] = project_files[i]
                    .clone()
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::Green));
            }
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Project Files")
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));

        let list = ratatui::widgets::List::new(project_files.clone())
            .block(block)
            // .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
            .highlight_symbol("> ");

        let list_state = &mut ListState::default().with_selected(Some(real_index));

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
