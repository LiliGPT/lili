use std::sync::Mutex;

use anyhow::Result;
use lilicore::code_analyst;
use ratatui::{
    prelude::{Backend, Rect},
    text::{Line, Span, Spans},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::app::AppState;

use super::super::{AppComponent, DrawableComponent};

pub struct ProjectInfoComponent {
    project_dir: String,
}

impl ProjectInfoComponent {
    pub fn new(project_dir: String) -> Result<Self> {
        Ok(Self { project_dir })
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::ProjectInfo(self))
    }
}

impl DrawableComponent for ProjectInfoComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Project Info")
            .padding(Padding::new(1, 1, 1, 1));
        let pathinfo = code_analyst::get_path_info(&self.project_dir).unwrap_or_default();
        let project_dir_line = Line::from(vec![Span::raw(format!(
            "Project Dir: {}",
            self.project_dir,
        ))]);
        let language_line = Line::from(vec![Span::raw(format!(
            "Language: {}",
            &pathinfo.code_language
        ))]);
        let framework_line = Line::from(vec![Span::raw(format!(
            "Framework: {}",
            &pathinfo.framework
        ))]);
        let dependencies_line = Line::from(vec![Span::raw(format!(
            "Dependencies Installed: {}",
            if pathinfo.dependencies_installed {
                "Yes"
            } else {
                "No"
            }
        ))]);
        let widget = Paragraph::new(vec![
            project_dir_line,
            language_line,
            framework_line,
            dependencies_line,
        ])
        .block(block);
        frame.render_widget(widget, rect);
        Ok(())
    }
}
