use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};

pub struct AvgProofTime {
    value: f64,
}

impl AvgProofTime {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Widget for AvgProofTime {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [top, bot] = Layout::vertical([Constraint::Min(1), Constraint::Length(1)])
            .areas(area.inner(Margin::new(1, 1)));

        Paragraph::new(vec![
            Line::styled("Average coSNARK generation time", Style::new().bold()),
            Line::raw("Current average coSNARK generation time in seconds"),
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
