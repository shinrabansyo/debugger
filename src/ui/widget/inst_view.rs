use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct InstView {
    selected: bool,
}

impl Widget for InstView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("This is body.")
            .centered()
            .block(self.gen_block())
            .render(area, buf);
    }
}

impl InstView {
    fn gen_block(&self) -> Block {
        if self.selected {
            Block::bordered()
                .title(Line::from(" Instructions ".bold()).centered())
                .border_set(border::THICK)
        } else {
            Block::bordered()
                .title(Line::from(" Instructions ".bold()).centered())
                .border_set(border::ROUNDED)
        }
    }
}

pub struct InstViewState {
    pub selected: bool,
}

impl InstViewState {
    pub fn new(selected: bool) -> Self {
        InstViewState {
            selected,
        }
    }

    pub fn gen_widget(&self) -> InstView {
        InstView {
            selected: self.selected,
        }
    }
}
