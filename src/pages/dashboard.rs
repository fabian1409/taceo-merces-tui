use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect, Size},
    style::Style,
    text::Line,
    widgets::{Paragraph, StatefulWidget, Widget},
};
use tui_scrollview::{ScrollView, ScrollViewState};

use crate::{
    LARGE_MIN, SinSignal,
    layout::{LayoutBuilder, Node},
    widgets::{
        avg_proof_time::AvgProofTime, avg_tx_latency::AvgTxLatency, mpc_log::MpcLog,
        network_util::NetworkUtil, node_util::NodeUtil, total_txs::TotalTxs, txs_log::TxsLog,
        txs_per_second::TxsPerSecond,
    },
};

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

pub struct Dashboard {
    pub scroll_view_state: ScrollViewState,
    window: [f64; 2],
    txs_per_second_signal: SinSignal,
    txs_per_second_data: Vec<(f64, f64)>,
    network_util: f64,
    node_cpu_signal: SinSignal,
    node_cpu_data: Vec<(f64, f64)>,
    node_net_up_signal: SinSignal,
    node_net_up_data: Vec<(f64, f64)>,
    node_net_down_signal: SinSignal,
    node_net_down_data: Vec<(f64, f64)>,
    tx_logs: Vec<(String, String)>,
    mpc_logs: Vec<(String, String)>,
}

impl Dashboard {
    pub fn new() -> Self {
        let mut txs_per_second_signal = SinSignal::new(0.1, 2.0, 10.0);
        let txs_per_second_data = txs_per_second_signal
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

        Self {
            scroll_view_state: ScrollViewState::default(),
            window: [0.0, 20.0],
            txs_per_second_signal,
            txs_per_second_data,
            network_util: 0.0,
            node_cpu_signal,
            node_cpu_data,
            node_net_up_signal,
            node_net_up_data,
            node_net_down_signal,
            node_net_down_data,
            tx_logs: TXS_LOGS
                .iter()
                .map(|e| (e.0.to_string(), e.1.to_string()))
                .collect(),
            mpc_logs: MPC_LOGS
                .iter()
                .map(|e| (e.0.to_string(), e.1.to_string()))
                .collect(),
        }
    }

    pub fn on_tick(&mut self) {
        self.window[0] += 1.0;
        self.window[1] += 1.0;

        self.txs_per_second_data.drain(0..10);
        self.txs_per_second_data
            .extend(self.txs_per_second_signal.by_ref().take(10));
        self.network_util =
            ((self.txs_per_second_data.last().unwrap().1 + 10.0) / 20.0).clamp(0.0, 1.0);
        self.node_cpu_data.drain(0..10);
        self.node_cpu_data
            .extend(self.node_cpu_signal.by_ref().take(10));
        self.node_net_up_data.drain(0..10);
        self.node_net_up_data
            .extend(self.node_net_up_signal.by_ref().take(10));
        self.node_net_down_data.drain(0..10);
        self.node_net_down_data
            .extend(self.node_net_down_signal.by_ref().take(10));

        self.tx_logs.rotate_left(1);
        self.mpc_logs.rotate_left(1);
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let height = if area.width >= LARGE_MIN {
            15 + 4 + 30 + 3 + 20
        } else {
            15 + 4 + 25 + 30 + 3 + 20 + 20
        };
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

        let layout = if area.width >= LARGE_MIN {
            LayoutBuilder::new()
                .child(
                    Node::vertical()
                        .child(Node::leaf("txs_graph").constraint(Constraint::Length(15)))
                        .child(Node::leaf("net_stats_title").constraint(Constraint::Length(4)))
                        .child(
                            Node::horizontal()
                                .spacing(1)
                                .child(
                                    Node::leaf("net_stats_gauge")
                                        .constraint(Constraint::Percentage(25)),
                                )
                                .child(
                                    Node::leaf("net_stats_graph")
                                        .constraint(Constraint::Percentage(50)),
                                )
                                .child(
                                    Node::vertical()
                                        .child("net_stats_values_0")
                                        .child("net_stats_values_1")
                                        .child("net_stats_values_2")
                                        .constraint(Constraint::Percentage(25)),
                                )
                                .constraint(Constraint::Length(30)),
                        )
                        .child(Node::leaf("live_logs_title").constraint(Constraint::Length(3)))
                        .child(
                            Node::horizontal()
                                .spacing(1)
                                .child("txs_logs")
                                .child("mpc_logs")
                                .constraint(Constraint::Length(20)),
                        ),
                )
                .build(area)
        } else {
            LayoutBuilder::new()
                .child(
                    Node::vertical()
                        .child(Node::leaf("txs_graph").constraint(Constraint::Length(15)))
                        .child(Node::leaf("net_stats_title").constraint(Constraint::Length(4)))
                        .child(Node::leaf("net_stats_graph").constraint(Constraint::Length(25)))
                        .child(
                            Node::horizontal()
                                .spacing(1)
                                .child("net_stats_gauge")
                                .child(
                                    Node::vertical()
                                        .child("net_stats_values_0")
                                        .child("net_stats_values_1")
                                        .child("net_stats_values_2"),
                                )
                                .constraint(Constraint::Length(30)),
                        )
                        .child(Node::leaf("live_logs_title").constraint(Constraint::Length(3)))
                        .child(Node::leaf("txs_logs").constraint(Constraint::Length(20)))
                        .child(Node::leaf("mpc_logs").constraint(Constraint::Length(20))),
                )
                .build(area)
        };

        TxsPerSecond::new(self.window, &self.txs_per_second_data).render(layout["txs_graph"], buf);

        Paragraph::new(vec![
            Line::raw(""),
            Line::styled("Network Stats", Style::new().bold()),
            Line::raw("Updated less than 10 seconds ago"),
        ])
        .render(layout["net_stats_title"], buf);

        NetworkUtil::new(self.network_util).render(layout["net_stats_gauge"], buf);

        NodeUtil::new(
            self.window,
            &self.node_cpu_data,
            &self.node_net_up_data,
            &self.node_net_down_data,
        )
        .render(layout["net_stats_graph"], buf);

        TotalTxs::new(1_000_000).render(layout["net_stats_values_0"], buf);
        AvgTxLatency::new(200.0).render(layout["net_stats_values_1"], buf);
        AvgProofTime::new(5.2).render(layout["net_stats_values_2"], buf);

        Paragraph::new(vec![
            Line::raw(""),
            Line::styled("Live Logs", Style::new().bold()),
        ])
        .render(layout["live_logs_title"], buf);

        TxsLog::new(&self.tx_logs).render(layout["txs_logs"], buf);
        MpcLog::new(&self.mpc_logs).render(layout["mpc_logs"], buf);

        scroll_view.render(
            scroll_view_area,
            scroll_view_buf,
            &mut self.scroll_view_state,
        );
    }
}
