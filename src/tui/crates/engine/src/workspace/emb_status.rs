use std::cell::RefCell;
use std::rc::Rc;

use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;

use sb_emu::Emulator;

use crate::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Status {
    workspace_name: String,
    input_mode: bool,
}

impl Widget for Status {
    fn draw(&self, _: &Rect, _: &Emulator) -> WidgetView {
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

        WidgetView::default().body(mode_line)
    }
}

impl Status {
    pub(crate) fn upcast(widget: &Rc<RefCell<Self>>) -> Rc<RefCell<dyn Widget>> {
        widget.clone()
    }

    pub(crate) fn set_workspace_name(&mut self, name: String) {
        self.workspace_name = name;
    }

    pub(crate) fn set_input_mode(&mut self, input_mode: bool) {
        self.input_mode = input_mode;
    }
}
