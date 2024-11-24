use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::Line;

use sb_emu::State as EmuState;

pub struct InstView {
    selected: bool,
    body: String,
}

impl Widget for InstView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
                .title(Line::from(" Instructions ".bold()).centered())
                .border_set(if self.selected { border::THICK } else { border::ROUNDED });

        Paragraph::new(self.body.as_str())
            .block(block)
            .render(area, buf);
    }
}

pub struct InstViewState {
    selected: bool,
    latest_event: String,
}

impl InstViewState {
    pub fn new(selected: bool, emu: &EmuState) -> Self {
        let mut state = InstViewState {
            selected,
            latest_event: String::new(),
        };
        state.update_emu(emu);
        state
    }

    pub fn gen_widget(&self) -> InstView {
        InstView {
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

    pub fn update_emu(&mut self, emu: &EmuState) {

    }
}
