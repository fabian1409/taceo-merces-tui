use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Size},
    style::Style,
    text::Line,
    widgets::{Paragraph, StatefulWidget, Widget},
};
use tui_scrollview::{ScrollView, ScrollViewState};

use crate::{
    SinSignal,
    widgets::{
        text_input::TextInputState,
        txs_per_second::TxsPerSecond,
        wallets_table::{WalletsTable, WalletsTableState},
    },
};

pub struct Wallets {
    pub scroll_view_state: ScrollViewState,
    window: [f64; 2],
    txs_per_second_signal: SinSignal,
    txs_per_second_data: Vec<(f64, f64)>,
    pub wallets_table_state: WalletsTableState,
}

impl Wallets {
    pub fn new() -> Self {
        let mut txs_per_second_signal = SinSignal::new(0.1, 2.0, 10.0);
        let txs_per_second_data = txs_per_second_signal
            .by_ref()
            .take(200)
            .collect::<Vec<(f64, f64)>>();
        Self {
            scroll_view_state: ScrollViewState::default(),
            window: [0.0, 20.0],
            txs_per_second_signal,
            txs_per_second_data,
            wallets_table_state: WalletsTableState {
                search_state: TextInputState::default().hint_text(" Search..."),
                search_focused: false,
            },
        }
    }

    pub fn on_tick(&mut self) {
        self.window[0] += 1.0;
        self.window[1] += 1.0;

        self.txs_per_second_data.drain(0..10);
        self.txs_per_second_data
            .extend(self.txs_per_second_signal.by_ref().take(10));
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let height = 75;
        let mut scroll_view = ScrollView::new(Size::new(area.width, height))
            .vertical_scrollbar_visibility(tui_scrollview::ScrollbarVisibility::Automatic)
            .horizontal_scrollbar_visibility(tui_scrollview::ScrollbarVisibility::Never);
        let scroll_view_area = area;
        let scroll_view_buf = buf;
        let mut area = *scroll_view.buf().area();
        if scroll_view_area.height < height {
            area.width -= 2; // adjust for scrollbar if vertical scrollbar is visible
        }
        let buf = scroll_view.buf_mut();

        let [txs_area, title_area, txs_table] = Layout::vertical([
            Constraint::Length(15),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(area);

        TxsPerSecond::new(self.window, &self.txs_per_second_data).render(txs_area, buf);

        Paragraph::new(vec![
            Line::raw(""),
            Line::styled("Wallets", Style::new().bold()),
        ])
        .render(title_area, buf);

        WalletsTable::new().render(txs_table, buf, &mut self.wallets_table_state);

        scroll_view.render(
            scroll_view_area,
            scroll_view_buf,
            &mut self.scroll_view_state,
        );
    }
}
