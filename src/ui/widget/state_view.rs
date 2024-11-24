use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct StateView {
    selected: bool,
    body: String,
}

impl Widget for StateView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.body.as_str())
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
    selected: bool,
    latest_event: String,
}

impl StateViewState {
    pub fn new(selected: bool) -> Self {
        StateViewState {
            selected,
            latest_event: String::new(),
        }
    }

    pub fn gen_widget(&self) -> StateView {
        StateView {
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
