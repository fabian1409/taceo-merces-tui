use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, List, ListItem, Widget},
};

pub struct MpcLog<'a> {
    logs: &'a [(String, String)],
}

impl<'a> MpcLog<'a> {
    pub fn new(logs: &'a [(String, String)]) -> Self {
        Self { logs }
    }
}

impl Widget for MpcLog<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [top, bot] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .spacing(1)
            .areas(area.inner(Margin::new(1, 1)));
        Line::styled("MPC Logs", Style::new().bold()).render(top, buf);

        let logs: Vec<ListItem> = self
            .logs
            .iter()
            .map(|(node, msg)| {
                let s = match node.as_str() {
                    "MPC Node 1" => Style::default().fg(Color::Cyan),
                    "MPC Node 2" => Style::default().fg(Color::Blue),
                    "MPC Node 3" => Style::default().fg(Color::Magenta),
                    "MPC Coordinator" => Style::default().fg(Color::Yellow),
                    _ => unreachable!(),
                };
                let content = vec![Line::from(vec![
                    Span::styled(node, s),
                    Span::raw(format!(": {msg}")),
                ])];
                ListItem::new(content)
            })
            .collect();
        List::new(logs).render(bot, buf);
    }
}
