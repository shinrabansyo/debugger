use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct MemView {
    selected: bool,
}

impl MemView {
    pub fn new(selected: bool) -> Self {
        MemView {
            selected,
        }
    }
}

impl Widget for MemView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("This is body.")
            .centered()
            .block(self.gen_block())
            .render(area, buf);
    }
}

impl MemView {
    fn gen_block(&self) -> Block {
        if self.selected {
            Block::bordered()
                .title(Line::from(" Memory ".bold()).centered())
                .border_set(border::THICK)
        } else {
            Block::bordered()
                .title(Line::from(" Memory ".bold()).centered())
                .border_set(border::ROUNDED)
        }
    }
}