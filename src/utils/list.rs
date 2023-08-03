use ratatui::widgets::ListItem;

#[derive(Default)]
pub struct SelectableList {
    pub selected_index: Option<usize>,
    pub items: Vec<(String, String)>,
}

impl SelectableList {
    pub fn new(items: Vec<(&str, &str)>) -> Self {
        let items = items
            .iter()
            .map(|(name, content)| (name.to_string(), content.to_string()))
            .collect();
        Self {
            items,
            selected_index: Some(0),
        }
    }

    pub fn to_items(&self) -> Vec<ListItem> {
        self.items
            .iter()
            .map(|item| ListItem::new(item.0.as_str()))
            .collect()
    }

    pub fn add_item(&mut self, key: &str, value: &str) {
        self.items.push((key.to_string(), value.to_string()));
    }

    pub fn len(&self) -> u16 {
        self.items.len() as u16
    }

    pub fn select_next(&mut self) {
        let selected_index = match self.selected_index {
            Some(index) => {
                if index >= self.items.len() - 1 {
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
                    self.items.len() - 1
                } else {
                    index - 1
                }
            }
            None => 0,
        };
        self.selected_index = Some(selected_index);
    }
}
