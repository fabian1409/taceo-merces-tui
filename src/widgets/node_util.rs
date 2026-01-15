use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize as _},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, BorderType, Chart, Dataset, Paragraph, Widget},
};

use crate::ACCENT_COLOR;

pub struct NodeUtil<'a> {
    window: [f64; 2],
    cpu: &'a [(f64, f64)],
    net_up: &'a [(f64, f64)],
    net_down: &'a [(f64, f64)],
}

impl<'a> NodeUtil<'a> {
    pub fn new(
        window: [f64; 2],
        cpu: &'a [(f64, f64)],
        net_up: &'a [(f64, f64)],
        net_down: &'a [(f64, f64)],
    ) -> Self {
        Self {
            window,
            cpu,
            net_up,
            net_down,
        }
    }
}

impl Widget for NodeUtil<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [top, bot] = Layout::vertical([Constraint::Length(4), Constraint::Fill(1)])
            .areas(area.inner(Margin::new(1, 1)));
        let [top_left, top_right] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(top);

        Paragraph::new(vec![
            Line::styled("Utilization of MPC nodes", Style::new().bold()),
            Line::raw("CPU usage and network throughput"),
        ])
        .render(top_left, buf);
        Paragraph::new(vec![
            Line::styled("1024 MiB", Style::new().bold()),
            Line::raw("Node memory usage"),
        ])
        .right_aligned()
        .render(top_right, buf);

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
                .name("CPU Usage in %")
                .marker(Marker::Braille)
                .style(Style::default().fg(ACCENT_COLOR))
                .data(self.cpu),
            Dataset::default()
                .name("Network Up in Mbps")
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Indexed(121)))
                .data(self.net_up),
            Dataset::default()
                .name("Network Down in Mbps")
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Indexed(123)))
                .data(self.net_down),
        ];

        Chart::new(datasets)
            .x_axis(Axis::default().labels(x_labels).bounds(self.window))
            .y_axis(
                Axis::default()
                    .labels(["-20".bold(), "0".into(), "20".bold()])
                    .bounds([-20.0, 20.0]),
            )
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Min(0)))
            .render(bot, buf);
    }
}
