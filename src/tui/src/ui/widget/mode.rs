use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;

use sb_emu::State as EmuState;

use crate::ui::widget::{Widget, WidgetState};

#[derive(Default)]
pub struct ModeState {
    input_mode: bool,
}

impl WidgetState for ModeState {
    fn draw(&self, _: &Rect, _: &EmuState) -> Widget {
        let mode_line = if self.input_mode {
            Line::from(vec![
                " Mode: ".into(),
                "INPUT".red().bold(),
            ])
        } else {
            Line::from(vec![
                " Mode: ".into(),
                "MOVE".green().bold(),
            ])
        };

        Widget::default().body(mode_line)
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        self.input_mode = match event.code {
            KeyCode::Esc => false,
            KeyCode::Char('i') => true,
            _ => self.input_mode,
        }
    }
}
