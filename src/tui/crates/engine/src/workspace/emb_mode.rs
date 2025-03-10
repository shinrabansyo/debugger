use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;

use sb_emu::State as EmuState;

use crate::widget::{Widget, WidgetState};

#[derive(Default)]
pub struct ModeState {
    workspace_name: String,
    input_mode: bool,
}

impl WidgetState for ModeState {
    fn draw(&self, _: &Rect, _: &EmuState) -> Widget {
        let mode_line = if self.input_mode {
            Line::from(vec![
                format!("{} / ", self.workspace_name).into(),
                "INPUT".red().bold(),
            ])
        } else {
            Line::from(vec![
                format!("{} / ", self.workspace_name).into(),
                "MOVE".green().bold(),
            ])
        };

        Widget::default().body(mode_line)
    }
}

impl ModeState {
    pub fn set_workspace_name(&mut self, name: String) {
        self.workspace_name = name;
    }

    pub fn set_input_mode(&mut self, input_mode: bool) {
        self.input_mode = input_mode;
    }
}
