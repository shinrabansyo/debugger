use std::cmp::max;

use ratatui::layout::Rect;
use ratatui::text::{Line, Text};
use crossterm::event::KeyEvent;

use sb_emu::State as EmuState;

use super::Device;

#[derive(Default)]
pub struct Uart;

impl Uart {
    pub fn gen_widget(&self, area: &Rect, emu: &EmuState) -> Device {
        let uart_lines = emu.devices.get_stat(0).unwrap();
        let uart_lines = uart_lines.lines().collect::<Vec<_>>();
        let view_begin_idx = max(0, (uart_lines.len() as i32) - (area.height as i32) + 2);
        let uart_output = uart_lines[view_begin_idx as usize..].join("\n");

        Device {
            selected: false,
            title: Line::raw(" Device 0: UART "),
            content: Text::raw(uart_output),
        }
    }

    pub fn handle_key_event(&mut self, _: KeyEvent) {}
}
