use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;

use sb_emu::State as EmuState;

use crate::ui::widget::{Widget, WidgetState};

#[derive(Default)]
pub struct HelpState;

impl WidgetState for HelpState {
    fn draw(&self,_: &Rect, _: &EmuState) -> Widget {
        let help_line = Line::from(vec![
            " Input Mode ".into(),
            "<i>".blue().bold(),
            " Move Mode ".into(),
            "<Esc> ".blue().bold(),
            "|".into(),
            " Up ".into(),
            "<k>".blue().bold(),
            " Down ".into(),
            "<j>".blue().bold(),
            " Left ".into(),
            "<h>".blue().bold(),
            " Right ".into(),
            "<l> ".blue().bold(),
            "|".into(),
            " Auto-Exec ".into(),
            "<Space>".blue().bold(),
            " Step ".into(),
            "<Enter> ".blue().bold(),
            "|".into(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);

        Widget::default().body(help_line.right_aligned())
    }

    fn handle_key_event(&mut self, _: KeyEvent) {}

    fn set_selected(&mut self, _: bool) {}
}
