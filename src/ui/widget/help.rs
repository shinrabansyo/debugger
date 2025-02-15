use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Paragraph};
use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct Help;

impl Widget for Help {
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
