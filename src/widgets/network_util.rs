use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};
use ratatui_circle_gauge::CircleGauge;

use crate::ACCENT_COLOR;

pub struct NetworkUtil {
    value: f64,
}

impl NetworkUtil {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Widget for NetworkUtil {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [top, _, bot, _] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Percentage(65),
            Constraint::Fill(1),
        ])
        .areas(area.inner(Margin::new(1, 1)));

        Paragraph::new(vec![
            Line::styled("Network utilization rate", Style::new().bold()),
            Line::raw("Amount of network's max throughput currently used"),
        ])
        .wrap(Wrap::default())
        .render(top, buf);

        CircleGauge::default()
            .ratio(self.value.clamp(0.0, 1.0))
            .stroke(5.0)
            .fill_style(Style::new().fg(ACCENT_COLOR))
            .empty_style(Style::new().dark_gray())
            .render(bot, buf);
    }
}
