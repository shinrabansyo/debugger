mod uart;
mod gpout;

use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};

use sb_emu::State as EmuState;

use crate::ui::widget::{Widget, WidgetState};
use uart::Uart;
use gpout::GPOut;

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

#[derive(Default)]
pub struct DeviceState {
    selected: bool,
    show_dev_id: u32,
    uart: Uart,
    gpout: GPOut,
}

impl WidgetState for DeviceState {
    type Widget = Device;

    fn affect(&self, emu: EmuState) -> EmuState {
        emu
    }

    fn draw(&self, _: &Rect, emu: &EmuState) -> Device {
        let mut device = match self.show_dev_id {
            0 => self.uart.gen_widget(emu),
            1 => self.gpout.gen_widget(emu),
            _ => unreachable!(),
        };
        device.selected = self.selected;
        device
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        const REGISTERED_DEVICES: u32 = 2;

        match event.code {
            KeyCode::Left => self.show_dev_id = max(0, self.show_dev_id - 1),
            KeyCode::Right => self.show_dev_id = min(REGISTERED_DEVICES - 1, self.show_dev_id + 1),
            _ => match self.show_dev_id {
                0 => self.uart.handle_key_event(event),
                1 => self.gpout.handle_key_event(event),
                _ => (),
            }
        }
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
