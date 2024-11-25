use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};

use sb_emu::State as EmuState;

pub struct DeviceView {
    selected: bool,
    title: Line<'static>,
    content: Text<'static>,
}

impl Widget for DeviceView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
                .title(self.title.bold().centered())
                .border_set(if self.selected { border::THICK } else { border::ROUNDED });

        Paragraph::new(self.content)
            .block(block)
            .render(area, buf);
    }
}

pub struct DeviceViewState {
    selected: bool,
    device_id: usize,
}

impl DeviceViewState {
    pub fn new(selected: bool) -> Self {
        DeviceViewState {
            selected,
            device_id: 0,
        }
    }

    pub fn gen_widget(&self, emu: &EmuState) -> DeviceView {
        let (title, content) = match self.device_id {
            0 => {
                let title = Line::raw(" Device 0: UART ");
                let content = Text::raw(emu.devices.get_stat(0).unwrap());
                (title, content)
            },
            _ => unreachable!(),
        };

        DeviceView {
            selected: self.selected,
            title,
            content,
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {}

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
