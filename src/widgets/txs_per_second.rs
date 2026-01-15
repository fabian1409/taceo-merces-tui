use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style, Stylize as _},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, BorderType, Chart, Dataset, Paragraph, Widget, Wrap},
};

use crate::ACCENT_COLOR;

pub struct TxsPerSecond<'a> {
    window: [f64; 2],
    data: &'a [(f64, f64)],
}

impl<'a> TxsPerSecond<'a> {
    pub fn new(window: [f64; 2], data: &'a [(f64, f64)]) -> Self {
        Self { window, data }
    }
}

impl Widget for TxsPerSecond<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [left, right] =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .areas(area.inner(Margin::new(1, 1)));
        let [left_top, left_bot] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(left);

        Paragraph::new(vec![
            Line::styled("Transactions", Style::new().bold()),
            Line::raw("Amount of transactions executed per second"),
        ])
        .wrap(Wrap::default())
        .render(left_top, buf);

        Line::default()
            .spans(vec![
                Span::styled(
                    format!("{:.2}", self.data.last().copied().unwrap_or_default().1),
                    Style::default().bold(),
                ),
                Span::raw(" tx/s"),
            ])
            .render(left_bot, buf);

        let x_labels = vec![
            Span::styled(
                format!("{}", self.window[0]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{}", (self.window[0] + self.window[1]) / 2.0)),
            Span::styled(
                format!("{}", self.window[1]),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ];
        let datasets = vec![
            Dataset::default()
                .marker(Marker::Braille)
                .style(Style::default().fg(ACCENT_COLOR))
                .data(self.data),
        ];

        Chart::new(datasets)
            .x_axis(Axis::default().labels(x_labels).bounds(self.window))
            .y_axis(
                Axis::default()
                    .labels(["-20".bold(), "0".into(), "20".bold()])
                    .bounds([-20.0, 20.0]),
            )
            .render(right, buf);
    }
}
