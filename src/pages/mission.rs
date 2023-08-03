use anyhow::{Error, Result};
use ratatui::{prelude::*, Frame};

use crate::components::DrawableComponent;

pub struct MissionPage {}

impl MissionPage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) -> Result<()> {
        let (top, bottom, left_top, left_mid, left_bottom, right) = self.layout(frame.size())?;
        Ok(())
    }

    fn layout(&mut self, frame_size: Rect) -> Result<(Rect, Rect, Rect, Rect, Rect, Rect)> {
        let [top_rect, main_rect, bottom_rect] = *Layout::default()
          .direction(Direction::Vertical)
          .constraints(
              [
                  Constraint::Length(1),
                  Constraint::Min(4),
                  Constraint::Length(1),
              ]
              .as_ref(),
          )
          .split(frame_size)
          else {
              return Err(Error::msg("Failed to split frame size"));
          };

        let [left_rect, right_rect] = *Layout::default()
          .direction(Direction::Horizontal)
          .horizontal_margin(0)
          .vertical_margin(0)
          .constraints([Constraint::Ratio(2, 6), Constraint::Ratio(4, 6)].as_ref())
          .split(main_rect)
          else {
              return Err(Error::msg("Failed to split main rect"));
          };

        let [left_top_rect, left_mid_rect, left_bottom_rect] = *Layout::default()
          .direction(Direction::Vertical)
          .constraints([
              Constraint::Min(5),
              Constraint::Length(5),
              Constraint::Length(5),
          ].as_ref())
          .split(left_rect)
          else {
              return Err(Error::msg("Failed to split left rect"));
          };

        Ok((
            top_rect,
            bottom_rect,
            left_top_rect,
            left_mid_rect,
            left_bottom_rect,
            right_rect,
        ))
    }
}
