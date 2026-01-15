use std::time::{Duration, Instant};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Offset, Rect, Size},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, List, ListState, Paragraph, StatefulWidget, Widget},
};
use ratatui_image::{Resize, picker::Picker};

use crate::pages::{dashboard::Dashboard, intro::Intro, wallets::Wallets};

mod layout;
mod pages;
mod widgets;

const LARGE_MIN: u16 = 120;

const ACCENT_COLOR: Color = Color::Indexed(122);

const SELECTED_STYLE: ratatui::style::Style = ratatui::style::Style::new()
    .bg(ACCENT_COLOR)
    .fg(Color::Black);

pub struct App {
    menu_state: ListState,
    show_menu: bool,
    should_exit: bool,
    image: ratatui_image::protocol::Protocol,
    intro: Intro,
    dashboard: Dashboard,
    wallets: Wallets,
}

impl App {
    pub fn new() -> eyre::Result<Self> {
        let mut menu_state = ListState::default();
        menu_state.select(Some(1));

        let picker = Picker::from_query_stdio()?;
        let dyn_img = image::ImageReader::open("logo.png")?.decode()?;
        let image = picker.new_protocol(dyn_img, Rect::new(0, 0, 4, 4), Resize::Scale(None))?;

        Ok(Self {
            menu_state,
            show_menu: true,
            should_exit: false,
            image,
            intro: Intro::new(),
            dashboard: Dashboard::new(),
            wallets: Wallets::new(),
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> eyre::Result<()> {
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

    fn scroll_down(&mut self) {
        match self.menu_state.selected() {
            Some(0) => self.intro.scroll_view_state.scroll_down(),
            Some(1) => self.dashboard.scroll_view_state.scroll_down(),
            Some(2) => self.wallets.scroll_view_state.scroll_down(),
            _ => unreachable!(),
        }
    }

    fn scroll_up(&mut self) {
        match self.menu_state.selected() {
            Some(0) => self.intro.scroll_view_state.scroll_up(),
            Some(1) => self.dashboard.scroll_view_state.scroll_up(),
            Some(2) => self.wallets.scroll_view_state.scroll_up(),
            _ => unreachable!(),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        // search box key handling
        if self.menu_state.selected() == Some(2) && self.wallets.wallets_table_state.search_focused
        {
            match key.code {
                KeyCode::Enter => {
                    // TODO search
                }
                KeyCode::Esc | KeyCode::Tab | KeyCode::BackTab => {
                    self.wallets.wallets_table_state.search_focused = false;
                }
                _ => {
                    self.wallets
                        .wallets_table_state
                        .search_state
                        .handle_events(key.code, key.modifiers);
                }
            }
            return;
        }

        // fall through to general key handling
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('m') => self.show_menu = !self.show_menu,
            KeyCode::Char('j') | KeyCode::Down | KeyCode::PageDown => {
                self.scroll_down();
            }
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.scroll_down();
            }
            KeyCode::Char('k') | KeyCode::Up | KeyCode::PageUp => {
                self.scroll_up();
            }
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.scroll_up();
            }
            KeyCode::Char('s') | KeyCode::Char('f') => {
                if self.menu_state.selected() == Some(2) {
                    self.wallets.wallets_table_state.search_focused = true;
                }
            }
            KeyCode::Tab => {
                if self.menu_state.selected() == Some(2) {
                    self.menu_state.select_first();
                } else {
                    self.menu_state.select_next();
                }
            }
            KeyCode::BackTab => {
                if self.menu_state.selected() == Some(0) {
                    self.menu_state.select_last();
                } else {
                    self.menu_state.select_previous();
                }
            }
            _ => {}
        }
    }

    fn on_tick(&mut self) {
        self.intro.on_tick();
        self.dashboard.on_tick();
        self.wallets.on_tick();
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
            Some(0) => self.intro.render(content, buf),
            Some(1) => self.dashboard.render(content, buf),
            Some(2) => self.wallets.render(content, buf),
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
        let list = List::new([
            "\t\n   Introduction\n\t",
            "\t\n   Dashboard\n\t",
            "\t\n   Wallets\n\t",
        ])
        .highlight_style(SELECTED_STYLE);
        StatefulWidget::render(list, area, buf, &mut self.menu_state);
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
