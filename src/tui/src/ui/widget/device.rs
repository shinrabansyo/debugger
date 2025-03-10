mod uart;
mod gpout;

use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;

use sb_emu::State as EmuState;

use crate::ui::widget::{Widget, WidgetState};
use uart::Uart;
use gpout::GPOut;

#[derive(Default)]
pub struct DeviceState {
    show_dev_id: u32,
    uart: Uart,
    gpout: GPOut,
}

impl WidgetState for DeviceState {
    fn draw(&self, area: &Rect, emu: &EmuState) -> Widget {
        match self.show_dev_id {
            0 => self.uart.draw(area, emu),
            1 => self.gpout.draw(emu),
            _ => unreachable!(),
        }
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
}
