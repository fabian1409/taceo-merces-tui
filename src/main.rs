use std::time::{Duration, Instant};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Margin, Offset, Rect, Size},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, Block, BorderType, Borders, Chart, Dataset, List, ListItem, ListState, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
};
use ratatui_circle_gauge::CircleGauge;
use ratatui_image::{Resize, picker::Picker};
use tui_scrollview::{ScrollView, ScrollViewState};

const ACCENT_COLOR: Color = Color::Indexed(122);

const SELECTED_STYLE: ratatui::style::Style = ratatui::style::Style::new()
    .bg(ACCENT_COLOR)
    .fg(Color::Black);

const TXS_LOGS: [(&str, &str); 13] = [
    ("block-0", "50 transactions published in 0x1234...abcd"),
    ("block-1", "50 transactions published in 0x1234...abcd"),
    ("block-2", "50 transactions published in 0x1234...abcd"),
    ("block-3", "50 transactions published in 0x1234...abcd"),
    ("block-4", "50 transactions published in 0x1234...abcd"),
    ("block-5", "50 transactions published in 0x1234...abcd"),
    ("block-6", "50 transactions published in 0x1234...abcd"),
    ("block-7", "50 transactions published in 0x1234...abcd"),
    ("block-8", "50 transactions published in 0x1234...abcd"),
    ("block-9", "50 transactions published in 0x1234...abcd"),
    ("block-10", "50 transactions published in 0x1234...abcd"),
    ("block-11", "50 transactions published in 0x1234...abcd"),
    ("block-12", "50 transactions published in 0x1234...abcd"),
];

const MPC_LOGS: [(&str, &str); 13] = [
    ("MPC Coordinator", "Sending job to MPC nodes"),
    (
        "MPC Node 1",
        "Received MPC job for a batch of transactions of size 50",
    ),
    (
        "MPC Node 2",
        "Received MPC job for a batch of transactions of size 50",
    ),
    (
        "MPC Node 3",
        "Received MPC job for a batch of transactions of size 50",
    ),
    ("MPC Node 1", "Finished connection establishment in 15ms"),
    ("MPC Node 2", "Finished connection establishment in 15ms"),
    ("MPC Node 3", "Finished connection establishment in 15ms"),
    ("MPC Node 1", "Start processing job.."),
    ("MPC Node 2", "Start processing job.."),
    ("MPC Node 3", "Start processing job.."),
    ("MPC Node 1", "Finished processing job in 500ms"),
    ("MPC Node 2", "Finished processing job in 500ms"),
    ("MPC Node 3", "Finished processing job in 500ms"),
];

fn main() -> eyre::Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new()?.run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
enum Focus {
    Menu,
    #[default]
    Content,
}

struct App {
    menu_state: ListState,
    show_menu: bool,
    intro_scroll_view_state: ScrollViewState,
    dashboard_scroll_view_state: ScrollViewState,
    wallet_scroll_view_state: ScrollViewState,
    should_exit: bool,
    tx_throughput_signal: SinSignal,
    tx_throughput_data: Vec<(f64, f64)>,
    node_cpu_signal: SinSignal,
    node_cpu_data: Vec<(f64, f64)>,
    node_net_up_signal: SinSignal,
    node_net_up_data: Vec<(f64, f64)>,
    node_net_down_signal: SinSignal,
    node_net_down_data: Vec<(f64, f64)>,
    window: [f64; 2],
    focus: Focus,
    image: ratatui_image::protocol::Protocol,
    tx_logs: Vec<(&'static str, &'static str)>,
    mpc_logs: Vec<(&'static str, &'static str)>,
}

impl App {
    fn new() -> eyre::Result<Self> {
        let mut menu_state = ListState::default();
        menu_state.select(Some(1));
        let mut tx_throughput_signal = SinSignal::new(0.1, 2.0, 10.0);
        let tx_throughput_data = tx_throughput_signal
            .by_ref()
            .take(200)
            .collect::<Vec<(f64, f64)>>();
        let mut node_cpu_signal = SinSignal::new(0.1, 3.0, 10.0);
        let node_cpu_data = node_cpu_signal
            .by_ref()
            .take(200)
            .collect::<Vec<(f64, f64)>>();
        let mut node_net_up_signal = SinSignal::new(0.1, 1.0, 10.0);
        let node_net_up_data = node_net_up_signal
            .by_ref()
            .take(200)
            .collect::<Vec<(f64, f64)>>();
        let mut node_net_down_signal = SinSignal::new(0.1, 2.5, 10.0);
        let node_net_down_data = node_net_down_signal
            .by_ref()
            .take(200)
            .collect::<Vec<(f64, f64)>>();
        let picker = Picker::from_query_stdio()?;

        let dyn_img = image::ImageReader::open("logo.png")?.decode()?;
        let image = picker.new_protocol(dyn_img, Rect::new(0, 0, 4, 4), Resize::Scale(None))?;

        Ok(Self {
            menu_state,
            show_menu: true,
            intro_scroll_view_state: ScrollViewState::default(),
            dashboard_scroll_view_state: ScrollViewState::default(),
            wallet_scroll_view_state: ScrollViewState::default(),
            should_exit: false,
            tx_throughput_signal,
            tx_throughput_data,
            node_cpu_signal,
            node_cpu_data,
            node_net_up_signal,
            node_net_up_data,
            node_net_down_signal,
            node_net_down_data,
            window: [0.0, 20.0],
            focus: Focus::default(),
            image,
            tx_logs: TXS_LOGS.to_vec(),
            mpc_logs: MPC_LOGS.to_vec(),
        })
    }
    fn run(mut self, mut terminal: DefaultTerminal) -> eyre::Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)?
                && let Event::Key(key) = event::read()?
            {
                self.handle_key(key);
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('m') => self.show_menu = !self.show_menu,
            KeyCode::Char('j') | KeyCode::Down => match self.focus {
                Focus::Menu => self.menu_state.select_next(),
                Focus::Content => match self.menu_state.selected() {
                    Some(0) => self.intro_scroll_view_state.scroll_down(),
                    Some(1) => self.dashboard_scroll_view_state.scroll_down(),
                    Some(2) => self.wallet_scroll_view_state.scroll_down(),
                    _ => unreachable!(),
                },
            },
            KeyCode::Char('k') | KeyCode::Up => match self.focus {
                Focus::Menu => self.menu_state.select_previous(),
                Focus::Content => match self.menu_state.selected() {
                    Some(0) => self.intro_scroll_view_state.scroll_up(),
                    Some(1) => self.dashboard_scroll_view_state.scroll_up(),
                    Some(2) => self.wallet_scroll_view_state.scroll_up(),
                    _ => unreachable!(),
                },
            },
            KeyCode::Tab => match self.focus {
                Focus::Menu => self.focus = Focus::Content,
                Focus::Content => self.focus = Focus::Menu,
            },
            _ => {}
        }
    }

    fn on_tick(&mut self) {
        self.tx_throughput_data.drain(0..10);
        self.tx_throughput_data
            .extend(self.tx_throughput_signal.by_ref().take(10));
        self.node_cpu_data.drain(0..10);
        self.node_cpu_data
            .extend(self.node_cpu_signal.by_ref().take(10));
        self.node_net_up_data.drain(0..10);
        self.node_net_up_data
            .extend(self.node_net_up_signal.by_ref().take(10));
        self.node_net_down_data.drain(0..10);
        self.node_net_down_data
            .extend(self.node_net_down_signal.by_ref().take(10));

        self.window[0] += 1.0;
        self.window[1] += 1.0;

        self.tx_logs.rotate_left(1);
        self.mpc_logs.rotate_left(1);
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();

        let content = if self.show_menu {
            let [menu, content] =
                Layout::horizontal([Constraint::Length(30), Constraint::Fill(1)]).areas(area);
            let [menu_top, mut menu_bot] =
                Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(menu);
            menu_bot.width -= 1; // Adjust for right border
            let [logo, title] =
                Layout::horizontal([Constraint::Length(5), Constraint::Fill(1)]).areas(menu_top);

            Block::new().borders(Borders::RIGHT).render(menu, buf);
            ratatui_image::Image::new(&self.image).render(logo, buf);
            self.title().render(title, buf);
            self.render_menu(menu_bot, buf);
            // hacky way to have left margin for content when menu is shown
            content
                .offset(Offset::new(1, 0))
                .resize(Size::new(content.width - 1, content.height))
        } else {
            area
        };

        match self.menu_state.selected() {
            Some(0) => self.render_intro(content, buf),
            Some(1) => self.render_dashboard(content, buf),
            Some(2) => self.render_dashboard(content, buf),
            _ => {}
        }
    }

    fn title(&self) -> impl Widget {
        Paragraph::new(vec![
            Line::styled("Merces by TACEO", Style::new().bold()),
            Line::raw("Confidential Tokens"),
        ])
    }

    fn render_menu(&mut self, area: Rect, buf: &mut Buffer) {
        let list = List::new(["  Introduction", "  Dashboard", "  Wallets"])
            .highlight_style(SELECTED_STYLE);
        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.menu_state);
    }

    fn render_intro(&mut self, area: Rect, buf: &mut Buffer) {
        let mut scroll_view = ScrollView::new(Size::new(area.width, 72))
            .vertical_scrollbar_visibility(tui_scrollview::ScrollbarVisibility::Automatic)
            .horizontal_scrollbar_visibility(tui_scrollview::ScrollbarVisibility::Never);
        let scroll_view_area = area;
        let scroll_view_buf = buf;
        let area = scroll_view.buf().area().inner(Margin::new(0, 1));
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
            &mut self.dashboard_scroll_view_state,
        );
    }

    fn render_txs_throughput_card(&mut self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [txs_left, txs_right] =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .areas(area.inner(Margin::new(1, 1)));
        let [txs_left_top, txs_left_bot] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(txs_left);

        Paragraph::new(vec![
            Line::styled("Transactions", Style::new().bold()),
            Line::raw("Amount of transactions executed per second"),
        ])
        .wrap(Wrap::default())
        .render(txs_left_top, buf);

        Line::default()
            .spans(vec![
                Span::styled(
                    format!(
                        "{:.2}",
                        self.tx_throughput_data
                            .last()
                            .copied()
                            .unwrap_or_default()
                            .1
                    ),
                    Style::default().bold(),
                ),
                Span::raw(" tx/s"),
            ])
            .render(txs_left_bot, buf);
        self.net_throughput_chart().render(txs_right, buf);
    }

    fn render_net_util_card(&mut self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [net_stats_left_top, _, net_stats_left_bot, _] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Percentage(65),
            Constraint::Fill(1),
        ])
        .areas(area.inner(Margin::new(1, 1)));

        Paragraph::new(vec![
            Line::styled("Network utilization rate", Style::new().bold()),
            Line::raw("Amount of network's max throughput currently used"),
        ])
        .wrap(Wrap::default())
        .render(net_stats_left_top, buf);
        CircleGauge::default()
            .ratio(0.25)
            .stroke(5.0)
            .fill_style(Style::new().fg(ACCENT_COLOR))
            .empty_style(Style::new().dark_gray())
            .render(net_stats_left_bot, buf);
    }

    fn render_nodes_util_card(&mut self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        let [net_stats_right_top, net_stats_right_bot] =
            Layout::vertical([Constraint::Length(4), Constraint::Fill(1)])
                .areas(area.inner(Margin::new(1, 1)));
        let [net_stats_right_top_left, net_stats_right_top_right] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(net_stats_right_top);

        Paragraph::new(vec![
            Line::styled("Utilization of MPC nodes", Style::new().bold()),
            Line::raw("CPU usage and network throughput"),
        ])
        .render(net_stats_right_top_left, buf);
        Paragraph::new(vec![
            Line::styled("1024 MiB", Style::new().bold()),
            Line::raw("Node memory usage"),
        ])
        .right_aligned()
        .render(net_stats_right_top_right, buf);

        self.node_util_chart().render(net_stats_right_bot, buf);
    }

    fn render_net_stats_values_card_0(&mut self, area: Rect, buf: &mut Buffer) {
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
        Line::styled("100,000,000", Style::default().bold()).render(bot, buf);
    }

    fn render_net_stats_values_card_1(&mut self, area: Rect, buf: &mut Buffer) {
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
                Span::styled("5.2", Style::default().bold()),
                Span::raw(" sec/tx"),
            ])
            .render(bot, buf);
    }

    fn render_net_stats_values_card_2(&mut self, area: Rect, buf: &mut Buffer) {
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
                Span::styled("5.2", Style::default().bold()),
                Span::raw(" sec/tx"),
            ])
            .render(bot, buf);
    }

    fn render_dashboard(&mut self, area: Rect, buf: &mut Buffer) {
        let mut scroll_view = ScrollView::new(Size::new(area.width, 90))
            .vertical_scrollbar_visibility(tui_scrollview::ScrollbarVisibility::Automatic)
            .horizontal_scrollbar_visibility(tui_scrollview::ScrollbarVisibility::Never);
        let scroll_view_area = area;
        let scroll_view_buf = buf;
        let mut area = *scroll_view.buf().area();
        area.width -= 2; // Adjust for scrollbar
        let buf = scroll_view.buf_mut();

        let [
            txs_area,
            net_stats_title,
            net_stats_area,
            net_stats_values_area,
            live_logs_title,
            txs_logs_area,
            mpc_logs_area,
        ] = Layout::vertical([
            Constraint::Length(12),
            Constraint::Length(4),
            Constraint::Length(25),
            Constraint::Length(12),
            Constraint::Length(3),
            Constraint::Length(15),
            Constraint::Length(15),
        ])
        .areas(area);

        self.render_txs_throughput_card(txs_area, buf);

        Paragraph::new(vec![
            Line::raw(""),
            Line::styled("Network Stats", Style::new().bold()),
            Line::raw("Updated less than 10 seconds ago"),
        ])
        .render(net_stats_title, buf);

        let [net_stats_left, net_stats_right] =
            Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)])
                .areas(net_stats_area);

        self.render_net_util_card(net_stats_left, buf);
        self.render_nodes_util_card(net_stats_right, buf);

        let [
            net_stats_values_left,
            net_stats_values_mid,
            net_stats_values_right,
        ] = Layout::horizontal([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .areas(net_stats_values_area);

        self.render_net_stats_values_card_0(net_stats_values_left, buf);
        self.render_net_stats_values_card_1(net_stats_values_mid, buf);
        self.render_net_stats_values_card_2(net_stats_values_right, buf);

        Paragraph::new(vec![
            Line::raw(""),
            Line::styled("Live Logs", Style::new().bold()),
        ])
        .render(live_logs_title, buf);

        self.txs_logs().render(txs_logs_area, buf);
        self.mpc_logs().render(mpc_logs_area, buf);

        scroll_view.render(
            scroll_view_area,
            scroll_view_buf,
            &mut self.dashboard_scroll_view_state,
        );
    }

    fn net_throughput_chart(&self) -> impl Widget {
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
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(ACCENT_COLOR))
                .data(&self.tx_throughput_data),
        ];

        Chart::new(datasets)
            .x_axis(Axis::default().labels(x_labels).bounds(self.window))
            .y_axis(
                Axis::default()
                    .labels(["-20".bold(), "0".into(), "20".bold()])
                    .bounds([-20.0, 20.0]),
            )
    }

    fn node_util_chart(&self) -> impl Widget {
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
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(ACCENT_COLOR))
                .data(&self.node_cpu_data),
            Dataset::default()
                .name("Network Up in Mbps")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Indexed(121)))
                .data(&self.node_net_up_data),
            Dataset::default()
                .name("Network Down in Mbps")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Indexed(123)))
                .data(&self.node_net_down_data),
        ];

        Chart::new(datasets)
            .x_axis(Axis::default().labels(x_labels).bounds(self.window))
            .y_axis(
                Axis::default()
                    .labels(["-20".bold(), "0".into(), "20".bold()])
                    .bounds([-20.0, 20.0]),
            )
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)))
    }

    fn txs_logs(&self) -> impl Widget {
        let logs: Vec<ListItem> = self
            .tx_logs
            .iter()
            .map(|&(block, msg)| {
                let content = vec![Line::from(vec![
                    Span::styled(block, Style::default().fg(Color::Green)),
                    Span::raw(format!(": {msg}")),
                ])];
                ListItem::new(content)
            })
            .collect();
        List::new(logs).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("Transaction Log"),
        )
    }

    fn mpc_logs(&self) -> impl Widget {
        let logs: Vec<ListItem> = self
            .mpc_logs
            .iter()
            .map(|&(node, msg)| {
                let s = match node {
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
        List::new(logs).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("MPC Log"),
        )
    }
}

#[derive(Clone)]
struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    const fn new(interval: f64, period: f64, scale: f64) -> Self {
        Self {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}
