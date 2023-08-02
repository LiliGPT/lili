use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout},
    widgets::{Paragraph, Widget},
    Frame,
};

use crate::components::{header::HeaderComponent, DrawableComponent};

pub struct MissionLayout {}

impl MissionLayout {
    pub fn render<B: Backend>(
        frame: &mut Frame<B>,
        top: HeaderComponent,
        left: Paragraph<'_>,
        right: Paragraph<'_>,
        bottom: Paragraph<'_>,
    ) {
        let [top_rect, main_rect, bottom_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    top.constraint(),
                    Constraint::Min(4),
                    bottom.constraint(),
                ]
                .as_ref(),
            )
            .split(frame.size())
            else {
                return;
            };

        let [left_rect, right_rect] = *Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(0)
            .vertical_margin(0)
            .constraints([left.constraint(), right.constraint()].as_ref())
            .split(main_rect)
            else {
                return;
            };

        top.render(frame, top_rect);
        left.render(frame, left_rect);
        right.render(frame, right_rect);
        bottom.render(frame, bottom_rect);
        // frame.render_widget(top.widget()., top_rect);
        // frame.render_widget(left.widget(), left_rect);
        // frame.render_widget(right.widget(), right_rect);
        // frame.render_widget(bottom.widget(), bottom_rect);
    }
}
