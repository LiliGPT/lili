use lilicore::code_missions_api::{MissionAction, MissionActionType};
use ratatui::widgets::ListItem;

#[derive(Debug, Clone, Default)]
pub struct SelectableList<T: SelectableItem> {
    pub selected_index: Option<usize>,
    pub items: Vec<T>,
}

pub trait SelectableItem {
    fn to_string(&self) -> String;
}

impl SelectableItem for (String, String) {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl SelectableItem for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}

impl SelectableItem for MissionAction {
    fn to_string(&self) -> String {
        let path = &self.path;
        let action_type = match self.action_type {
            MissionActionType::CreateFile => "+",
            MissionActionType::UpdateFile => "~",
            // MissionActionType::DeleteFile => "D",
        };
        format!("{} {}", action_type, path)
    }
}

impl<T: SelectableItem> SelectableList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            selected_index: Some(0),
        }
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        match self.selected_index {
            Some(index) => self.items.get(index),
            None => None,
        }
    }

    pub fn to_items(&self) -> Vec<ListItem> {
        self.items
            .iter()
            .map(|item| ListItem::new(item.to_string()))
            .collect()
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected_index = index;
    }

    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn len(&self) -> u16 {
        self.items.len() as u16
    }

    pub fn select_next(&mut self) {
        let selected_index = match self.selected_index {
            Some(index) => {
                let items_len = self.items.len();
                if items_len <= 0 {
                    0
                } else if index >= items_len - 1 {
                    0
                } else {
                    index + 1
                }
            }
            None => 0,
        };
        self.selected_index = Some(selected_index);
    }

    pub fn select_previous(&mut self) {
        let selected_index = match self.selected_index {
            Some(index) => {
                if index == 0 {
                    let items_len = self.items.len();
                    if items_len <= 0 {
                        0
                    } else {
                        items_len - 1
                    }
                } else {
                    index - 1
                }
            }
            None => 0,
        };
        self.selected_index = Some(selected_index);
    }
}
