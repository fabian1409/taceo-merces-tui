use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, Widget},
};

pub struct TxsLog<'a> {
    logs: &'a [(String, String)],
}

impl<'a> TxsLog<'a> {
    pub fn new(logs: &'a [(String, String)]) -> Self {
        Self { logs }
    }
}

impl Widget for TxsLog<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [top, bot] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .spacing(1)
            .areas(area.inner(Margin::new(1, 1)));
        Line::styled("Transaction Logs", Style::new().bold()).render(top, buf);

        let logs: Vec<ListItem> = self
            .logs
            .iter()
            .map(|(block, msg)| {
                let content = vec![Line::from(vec![
                    Span::styled(block, Style::default().fg(Color::Green)),
                    Span::raw(format!(": {msg}")),
                ])];
                ListItem::new(content)
            })
            .collect();
        List::new(logs).render(bot, buf);
    }
}
