use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Cell, Row, StatefulWidget, Table, Widget},
};

use crate::{
    ACCENT_COLOR,
    widgets::text_input::{TextInput, TextInputState},
};

pub struct WalletsTableState {
    pub search_state: TextInputState,
    pub search_focused: bool,
}

pub struct WalletsTable {}

impl WalletsTable {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for WalletsTable {
    type State = WalletsTableState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [search_area, table_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
        let [search_area] = Layout::horizontal([Constraint::Max(50)]).areas(search_area);

        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(search_area, buf);

        let search_textbox = TextInput::default().render_cursor(state.search_focused);
        StatefulWidget::render(
            search_textbox,
            search_area.inner(Margin::new(1, 1)),
            buf,
            &mut state.search_state,
        );

        let header = ["Wallet", "Last Transfer", "Transferred Amount", "Balance"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(Style::new().fg(Color::DarkGray))
            .top_margin(1)
            .height(3);
        let rows = (0..20)
            .map(|i| {
                let wallet = format!("0x1234...{:04x}", i);
                let last_transfer = format!("2024/09/{:02} 12:34", i + 1);
                let transferred_amount = Line::default().spans([
                    Span::raw(format!("{}\t\t\t", (i + 1) * 1000)),
                    Span::styled(" ", Style::default().fg(ACCENT_COLOR)),
                ]);
                let balance = Line::default().spans([
                    Span::raw(format!("{}\t\t\t", (20 - i) * 5000)),
                    Span::styled(" ", Style::default().fg(ACCENT_COLOR)),
                ]);
                Row::new(vec![
                    Cell::from(wallet),
                    Cell::from(last_transfer),
                    Cell::from(transferred_amount),
                    Cell::from(balance),
                ])
                .top_margin(1)
                .height(3)
            })
            .collect::<Vec<Row>>();
        let widths = [
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(Block::bordered().border_type(BorderType::Rounded));
        Widget::render(table, table_area, buf);
    }
}
