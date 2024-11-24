use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct MemView {
    selected: bool,
    body: String,
}

impl Widget for MemView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
                .title(Line::from(" Memory ".bold()).centered())
                .border_set(if self.selected { border::THICK } else { border::ROUNDED });

        Paragraph::new(self.body.as_str())
            .block(block)
            .render(area, buf);
    }
}

pub struct MemViewState {
    selected: bool,
    latest_event: String,
}

impl MemViewState {
    pub fn new(selected: bool) -> Self {
        MemViewState {
            selected,
            latest_event: String::new(),
        }
    }

    pub fn gen_widget(&self) -> MemView {
        MemView {
            selected: self.selected,
            body: self.latest_event.clone(),
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        self.latest_event = format!("{:?}", event);
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
