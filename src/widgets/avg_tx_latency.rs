use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};

pub struct AvgTxLatency {
    value: f64,
}

impl AvgTxLatency {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Widget for AvgTxLatency {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [top, bot] = Layout::vertical([Constraint::Min(1), Constraint::Length(1)])
            .areas(area.inner(Margin::new(1, 1)));

        Paragraph::new(vec![
            Line::styled("Average transaction latency", Style::new().bold()),
            Line::raw("Current average transaction time in seconds"),
        ])
        .wrap(Wrap::default())
        .render(top, buf);
        Line::default()
            .spans(vec![
                Span::styled(format!("{:.1}", self.value), Style::default().bold()),
                Span::raw(" sec/tx"),
            ])
            .render(bot, buf);
    }
}
