use std::sync::Mutex;

use anyhow::Result;
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;

use super::{AppComponent, DrawableComponent};

#[derive(Debug, PartialEq, Clone, Default)]
pub enum HeaderStatus {
    #[default]
    Idle,
    Loading,
    ErrorMessage(String),
    LoadingMessage(String),
    SuccessMessage(String),
}

pub struct HeaderComponent;

impl HeaderComponent {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::Header(self))
    }
}

impl DrawableComponent for HeaderComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let mut texts: Vec<Span> = vec![];
        texts.push(Span::styled(
            state.project_dir.split("/").last().unwrap(),
            ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
        ));
        texts.push(Span::raw("        "));
        texts.push(Span::raw("        "));

        let loading_text = match state.header_status {
            HeaderStatus::Idle => "Idle",
            HeaderStatus::Loading => "Loading",
            HeaderStatus::ErrorMessage(ref msg) => msg,
            HeaderStatus::LoadingMessage(ref msg) => msg,
            HeaderStatus::SuccessMessage(ref msg) => msg,
        };
        let loading_color = match state.header_status {
            HeaderStatus::Idle => ratatui::style::Color::DarkGray,
            HeaderStatus::Loading => ratatui::style::Color::Cyan,
            HeaderStatus::ErrorMessage(_) => ratatui::style::Color::Red,
            HeaderStatus::LoadingMessage(_) => ratatui::style::Color::LightCyan,
            HeaderStatus::SuccessMessage(_) => ratatui::style::Color::Green,
        };
        let prefix_text = match state.header_status {
            HeaderStatus::Idle => "*",
            HeaderStatus::Loading => "*",
            HeaderStatus::ErrorMessage(_) => "Error:",
            HeaderStatus::LoadingMessage(_) => "Loading:",
            HeaderStatus::SuccessMessage(_) => "",
        };
        texts.push(Span::styled(
            format!("{} {}", prefix_text, loading_text),
            ratatui::style::Style::default().fg(loading_color),
        ));
        texts.push(Span::raw("        "));
        texts.push(Span::raw("        "));
        texts.push(Span::styled(
            format!("{}", state.user_name),
            ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
        ));
        let header = Paragraph::new(Line::from(texts))
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left);
        frame.render_widget(header, rect);
        Ok(())
    }
}
