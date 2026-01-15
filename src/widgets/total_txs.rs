use num_format::{Locale, ToFormattedString};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};

pub struct TotalTxs {
    value: u64,
}

impl TotalTxs {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

impl Widget for TotalTxs {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [top, bot] = Layout::vertical([Constraint::Min(1), Constraint::Length(1)])
            .areas(area.inner(Margin::new(1, 1)));

        Paragraph::new(vec![
            Line::styled("Total transactions", Style::new().bold()),
            Line::raw("Total number of transactions"),
        ])
        .wrap(Wrap::default())
        .render(top, buf);
        Line::styled(
            self.value.to_formatted_string(&Locale::en),
            Style::default().bold(),
        )
        .render(bot, buf);
    }
}
