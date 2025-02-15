use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};

use sb_emu::State as EmuState;

use crate::ui::widget::Widget;

pub struct Device {
    selected: bool,
    title: Line<'static>,
    content: Text<'static>,
}

impl Widget for Device {
    type State = DeviceState;
}

impl ratatui::widgets::Widget for Device {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
                .title(self.title.bold().centered())
                .border_set(if self.selected { border::THICK } else { border::ROUNDED });

        Paragraph::new(self.content)
            .block(block)
            .render(area, buf);
    }
}

pub struct DeviceState {
    selected: bool,
    device_id: i32,
}

impl Default for DeviceState {
    fn default() -> Self {
        DeviceState {
            selected: false,
            device_id: 0,
        }
    }
}

impl DeviceState {
    pub fn gen_widget(&self, emu: &EmuState) -> Device {
        let (title, content) = match self.device_id {
            0 => {
                let title = Line::raw(" Device 0: UART ");
                let content = Text::raw(emu.devices.get_stat(0).unwrap());
                (title, content)
            },
            1 => {
                let title = Line::raw(" Device 1: GPIO ");
                let content = Text::raw(emu.devices.get_stat(4).unwrap());
                (title, content)
            }
            _ => unreachable!(),
        };

        Device {
            selected: self.selected,
            title,
            content,
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        const REGISTERED_DEVICES: i32 = 2;

        self.device_id = match event.code {
            KeyCode::Left => max(0, self.device_id - 1),
            KeyCode::Right => min(REGISTERED_DEVICES - 1, self.device_id + 1),
            _ => self.device_id,
        };
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
