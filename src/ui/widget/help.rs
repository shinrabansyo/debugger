use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::style::Stylize;
use ratatui::text::Line;

use sb_emu::State as EmuState;

use crate::ui::widget::{Widget, WidgetState};

pub struct Help;

impl Widget for Help {
    type State = HelpState;
}

impl ratatui::widgets::Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let help_line = Line::from(vec![
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

        Paragraph::new(help_line.centered()).render(area, buf);
    }
}

#[derive(Default)]
pub struct HelpState;

impl WidgetState for HelpState {
    type Widget = Help;

    fn draw(&self, _: &EmuState) -> Help {
        Help
    }

    fn handle_key_event(&mut self, _: KeyEvent) {}

    fn set_selected(&mut self, _: bool) {}
}
