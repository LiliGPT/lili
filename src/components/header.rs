use anyhow::Result;
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::DrawableComponent;

#[derive(Debug, PartialEq, Clone)]
pub enum HeaderStatus {
    Idle,
    Loading,
}

pub struct HeaderComponent {
    pub project_name: String,
    status: HeaderStatus,
}

impl HeaderComponent {
    pub fn new(project_name: String) -> Result<Self> {
        let status = HeaderStatus::Idle;
        Ok(Self {
            project_name,
            status,
        })
    }

    pub fn set_status(&mut self, status: HeaderStatus) {
        self.status = status;
    }

    pub fn get_status(&self) -> HeaderStatus {
        self.status.clone()
    }
}

impl DrawableComponent for HeaderComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) -> Result<()> {
        let mut texts: Vec<Span> = vec![];
        texts.push(Span::styled(
            format!("{} ", self.project_name),
            ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
        ));
        texts.push(Span::raw("        "));
        texts.push(Span::raw("        "));
        texts.push(Span::raw("        "));

        let loading_text = match self.status {
            HeaderStatus::Idle => "Idle",
            HeaderStatus::Loading => "Loading",
        };
        let loading_color = match self.status {
            HeaderStatus::Idle => ratatui::style::Color::DarkGray,
            HeaderStatus::Loading => ratatui::style::Color::Cyan,
        };
        texts.push(Span::styled(
            format!("* {}", loading_text),
            ratatui::style::Style::default().fg(loading_color),
        ));
        let header = Paragraph::new(Line::from(texts))
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left);
        f.render_widget(header, rect);
        Ok(())
    }
}
