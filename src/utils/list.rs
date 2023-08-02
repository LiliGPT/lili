use ratatui::widgets::ListItem;

#[derive(Default)]
pub struct BatchList {
    items: Vec<String>,
}

impl BatchList {
    pub fn new(items: Vec<&str>) -> Self {
        Self {
            items: items.iter().map(|item| item.to_string()).collect(),
        }
    }

    pub fn to_str(&self) -> &[String] {
        &self.items
    }

    pub fn to_items(&self) -> Vec<ListItem> {
        self.items
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect()
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.to_string());
    }

    pub fn len(&self) -> u16 {
        self.items.len() as u16
    }
}
