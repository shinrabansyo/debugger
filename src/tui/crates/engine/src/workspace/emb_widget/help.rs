use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;

use sb_dbg::Debugger;

use crate::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Help;

impl Widget for Help {
    fn draw(&self,_: &Rect, _: &Debugger) -> WidgetView {
        let help_line = Line::from(vec![
            " Control Mode ".into(),
            "<i>".blue().bold(),
            " Move Mode ".into(),
            "<Esc> ".blue().bold(),
            "|".into(),
            " Command ".into(),
            "<Enter> ".blue().bold(),
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
            "<s> ".blue().bold(),
            "Continue ".into(),
            "<c> ".blue().bold(),
            "|".into(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ]);

        WidgetView::default().body(help_line.right_aligned())
    }
}
