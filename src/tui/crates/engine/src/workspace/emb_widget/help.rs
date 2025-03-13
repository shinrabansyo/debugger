use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;

use sb_emu::Emulator;

use crate::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Help;

impl Widget for Help {
    fn draw(&self,_: &Rect, _: &Emulator) -> WidgetView {
        let help_line = Line::from(vec![
            " Control Mode ".into(),
            "<i>".blue().bold(),
            " Move Mode ".into(),
            "<Esc> ".blue().bold(),
            "|".into(),
            " Command ".into(),
            "<c> ".blue().bold(),
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

        WidgetView::default().body(help_line.right_aligned())
    }
}
