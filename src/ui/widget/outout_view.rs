use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct OutputView {
    selected: bool,
    body: String,
}

impl Widget for OutputView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.body.as_str())
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
    selected: bool,
    latest_event: String,
}

impl OutputViewState {
    pub fn new(selected: bool) -> Self {
        OutputViewState {
            selected,
            latest_event: String::new(),
        }
    }

    pub fn gen_widget(&self) -> OutputView {
        OutputView {
            selected: self.selected,
            body: self.latest_event.clone(),
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        self.latest_event = format!("{:?}", event);
    }
}
