use std::sync::Mutex;

use anyhow::Result;
use ratatui::{
    prelude::{Alignment, Backend, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{AppScreen, AppState, FocusedBlock};

use super::{AppComponent, DrawableComponent};

pub struct ShortcutsComponent {}

impl ShortcutsComponent {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn as_mutex(self) -> Mutex<AppComponent> {
        Mutex::new(AppComponent::Shortcuts(self))
    }

    fn get_shortcuts(&self, screen: &AppScreen, focused_block: &FocusedBlock) -> Vec<(&str, &str)> {
        match screen {
            AppScreen::SignIn => {
                return vec![
                    ("Esc", "exit"),
                    ("Tab", "next field"),
                    ("Shift+Tab", "previous field"),
                    ("Enter", "submit"),
                ];
            }
            AppScreen::CreateTempBranch => {
                return vec![
                    ("Esc", "do NOT create branch"),
                    ("Enter", "create"),
                    ("q", "quit"),
                ];
            }
            _ => {}
        }

        match focused_block {
            FocusedBlock::Home => {
                return vec![
                    ("i", "create mission"),
                    ("c", "context"),
                    ("r", "reset"),
                    ("g", "git"),
                    ("s", "settings"),
                    ("h", "help"),
                ]
            }
            FocusedBlock::Message => return vec![("Esc", "exit"), ("Enter", "send")],
            FocusedBlock::Actions => return vec![("a", "approve and run"), ("x", "cancel")],
            _ => {}
        }

        vec![]
    }
}

impl DrawableComponent for ShortcutsComponent {
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        // let shortcuts = self
        //     .get_shortcuts()
        //     .iter()
        //     .map(|(key, action)| format!("{}) {}", key, action))
        //     .collect::<Vec<String>>()
        //     .join("      ");
        let mut innerp: Vec<Span> = vec![];

        self.get_shortcuts(&state.screen, &state.focused_block)
            .iter()
            .for_each(|(key, action)| {
                innerp.push(Span::styled(
                    format!("{}", key),
                    ratatui::style::Style::default().fg(ratatui::style::Color::White),
                ));
                innerp.push(Span::styled(
                    format!(" {}", action),
                    ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
                ));
                innerp.push(Span::raw("      "));
            });

        let paragraph = Paragraph::new(Line::from(innerp))
            .block(Block::default().borders(Borders::NONE))
            .alignment(Alignment::Left);
        frame.render_widget(paragraph, rect);
        Ok(())
    }
}
