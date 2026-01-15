use ratatui::{
    buffer::Buffer,
    layout::{Rect, Size},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, StatefulWidget, Widget, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState};

pub struct Intro {
    pub scroll_view_state: ScrollViewState,
}

impl Intro {
    pub fn new() -> Self {
        Self {
            scroll_view_state: ScrollViewState::default(),
        }
    }

    pub fn on_tick(&mut self) {}

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        {
            let height = 20;
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

            Paragraph::new(vec![
                Line::styled("Welcome to Merces", Style::new().bold()),
                Line::raw(""),
                Line::raw(""),
                Line::default().spans(vec![
                    Span::raw("Merces by TACEO is the first implementation of Private Shared State (PSS) that demonstrates a "),
                    Span::styled("confidential stablecoin transfer system. ", Style::new().bold()),
                    Span::raw("It is currently deployed on Base and Arc testnet, with more networks on the way.")
                ]),
                Line::raw(""),
                Line::raw("Stablecoin rails are powerful, but today every transfer is public, leaving a complete trail of financial history visible to anyone."),
                Line::raw(""),
                Line::raw("With private transfers, everyday use cases like payroll, vendor payments, subscription services, and cross-border settlements finally become viable without exposing sensitive financial data to the entire world."),
                Line::raw(""),
                Line::default().spans(vec![
                    Span::raw("Over the coming weeks, this system will issue "),
                    Span::styled("tens of millions of transactions ", Style::new().bold()),
                    Span::raw("to demonstrate its resilience under heavy load.")
                ]),
                Line::raw(""),
                Line::default().spans(vec![
                    Span::raw("To demonstrate utilization, user wallets are continuously served with background activity, showing a live, confidential stablecoin network capable of pushing "),
                    Span::styled("up to ~200 TPS,", Style::new().bold()),
                    Span::raw("comparable to what PayPal achieves today.")
                ]),
                Line::raw(""),
                Line::raw("For a deeper dive into the motivation behind this project, read the full explanation here: Blog Post"),
            ]).wrap(Wrap::default()).render(area, buf);

            scroll_view.render(
                scroll_view_area,
                scroll_view_buf,
                &mut self.scroll_view_state,
            );
        }
    }
}
