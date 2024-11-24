use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct OutputView;

impl Widget for OutputView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Output ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        Paragraph::new("This is body.")
            .centered()
            .block(block)
            .render(area, buf);
    }
}
