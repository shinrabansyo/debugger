use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::style::Stylize;
use ratatui::text::Line;

use sb_emu::State as EmuState;

use crate::ui::widget::{Widget, WidgetState};

pub struct Mode {
    input_mode: bool,
}

impl Widget for Mode {
    type State = ModeState;
}

impl ratatui::widgets::Widget for Mode {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

        Paragraph::new(mode_line.left_aligned()).render(area, buf);
    }
}

#[derive(Default)]
pub struct ModeState {
    input_mode: bool,
}

impl WidgetState for ModeState {
    type Widget = Mode;

    fn affect(&self, emu: EmuState) -> EmuState {
        emu
    }

    fn draw(&self, _: &Rect, _: &EmuState) -> Mode {
        Mode {
            input_mode: self.input_mode,
        }
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        self.input_mode = match event.code {
            KeyCode::Esc => false,
            KeyCode::Char('i') => true,
            _ => self.input_mode,
        }
    }

    fn set_selected(&mut self, _: bool) {}
}
