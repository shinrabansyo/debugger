use ratatui::text::{Line, Text};
use crossterm::event::KeyEvent;

use sb_emu::State as EmuState;

use super::Device;

#[derive(Default)]
pub struct Uart;

impl Uart {
    pub fn gen_widget(&self, emu: &EmuState) -> Device {
        Device {
            selected: false,
            title: Line::raw(" Device 0: UART "),
            content: Text::raw(emu.devices.get_stat(0).unwrap()),
        }
    }

    pub fn handle_key_event(&mut self, _: KeyEvent) {}
}
