use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyModifiers},
    layout::Rect,
    style::{Modifier, Style},
    widgets::StatefulWidget,
};

#[derive(Default)]
pub struct TextInput {
    style: Style,
    hint_style: Style,
    render_cursor: bool,
}

impl TextInput {
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn hint_style(mut self, hint_style: Style) -> Self {
        self.hint_style = hint_style;
        self
    }

    pub fn render_cursor(mut self, render: bool) -> Self {
        self.render_cursor = render;
        self
    }
}

pub struct TextInputState {
    pub cursor_pos: usize,
    pub text: String,
    pub hint_text: Option<String>,
    start: usize,
}

impl Default for TextInputState {
    fn default() -> Self {
        Self {
            cursor_pos: Default::default(),
            text: Default::default(),
            hint_text: Some("<hint text>".to_string()),
            start: 0,
        }
    }
}

impl TextInputState {
    pub fn hint_text<S: Into<String>>(mut self, hint_text: S) -> Self {
        self.hint_text = Some(hint_text.into());
        self
    }

    pub fn handle_events(&mut self, key_code: KeyCode, key_modifiers: KeyModifiers) {
        match (key_code, key_modifiers) {
            (KeyCode::Left, _) => {
                self.cursor_pos = if self.cursor_pos > 0 {
                    self.cursor_pos - 1
                } else {
                    self.cursor_pos
                };
            }
            (KeyCode::Right, _) => {
                self.cursor_pos = if self.cursor_pos < self.text.len() {
                    self.cursor_pos + 1
                } else {
                    self.text.len()
                };
            }
            (KeyCode::Backspace, _) => {
                if self.cursor_pos > 0 {
                    self.cursor_pos = std::cmp::max(self.cursor_pos - 1, 0);
                    self.text.remove(self.cursor_pos);
                }
            }
            (KeyCode::Delete, _) => {
                if self.cursor_pos < self.text.len() {
                    self.text.remove(self.cursor_pos);

                    if self.cursor_pos == self.text.len() && !self.text.is_empty() {
                        self.cursor_pos -= 1;
                    }
                }
            }
            (KeyCode::Char(x), _) => {
                self.text.insert(self.cursor_pos, x);
                self.cursor_pos += 1;
            }
            (_, _) => {}
        }
    }
}

impl StatefulWidget for TextInput {
    type State = TextInputState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        if !state.text.is_empty() {
            let w = usize::from(area.width) - 1;
            if state.cursor_pos > state.start + w {
                state.start = state.cursor_pos - w;
            };

            if state.cursor_pos < state.start {
                state.start = state.cursor_pos;
            }

            let end = std::cmp::min(state.start + w + 1, state.text.len());

            let visible_text = &state.text[state.start..end];
            buf.set_string(area.x, area.y, visible_text, self.style);
        } else if let Some(hint) = state.hint_text.as_ref() {
            buf.set_string(area.x, area.y, hint, self.hint_style);
        }

        if self.render_cursor && !state.text.is_empty() {
            let pos_char = state
                .text
                .chars()
                .nth(state.cursor_pos)
                .or(state
                    .hint_text
                    .as_ref()
                    .and_then(|s| s.chars().nth(state.cursor_pos)))
                .unwrap_or(' ');
            let cur_pos = u16::try_from(state.cursor_pos.saturating_sub(state.start)).unwrap_or(0);

            buf.set_string(
                area.x + cur_pos,
                area.y,
                format!("{}", &pos_char),
                self.style.add_modifier(Modifier::REVERSED),
            );
        }
    }
}
