use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct StateView {
    selected: bool,
}

impl Widget for StateView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("This is body.")
            .centered()
            .block(self.gen_block())
            .render(area, buf);
    }
}

impl StateView {
    fn gen_block(&self) -> Block {
        if self.selected {
            Block::bordered()
                .title(Line::from(" State ".bold()).centered())
                .border_set(border::THICK)
        } else {
            Block::bordered()
                .title(Line::from(" State ".bold()).centered())
                .border_set(border::ROUNDED)
        }
    }
}

pub struct StateViewState {
    pub selected: bool,
}

impl StateViewState {
    pub fn new(selected: bool) -> Self {
        StateViewState {
            selected,
        }
    }

    pub fn gen_widget(&self) -> StateView {
        StateView {
            selected: self.selected,
        }
    }
}
