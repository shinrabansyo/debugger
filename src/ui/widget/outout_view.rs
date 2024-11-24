use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct OutputView {
    selected: bool,
}

impl Widget for OutputView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("This is body.")
            .centered()
            .block(self.gen_block())
            .render(area, buf);
    }
}

impl OutputView {
    fn gen_block(&self) -> Block {
        if self.selected {
            Block::bordered()
                .title(Line::from(" Output ".bold()).centered())
                .border_set(border::THICK)
        } else {
            Block::bordered()
                .title(Line::from(" Output ".bold()).centered())
                .border_set(border::ROUNDED)
        }
    }
}

pub struct OutputViewState {
    pub selected: bool,
}

impl OutputViewState {
    pub fn new(selected: bool) -> Self {
        OutputViewState {
            selected,
        }
    }

    pub fn gen_widget(&self) -> OutputView {
        OutputView {
            selected: self.selected,
        }
    }
}
