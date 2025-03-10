use std::cmp::max;

use ratatui::layout::Rect;
use ratatui::text::Text;
use crossterm::event::KeyEvent;

use sb_emu::State as EmuState;

use crate::ui::widget::Widget;

#[derive(Default)]
pub struct Uart;

impl Uart {
    pub fn draw(&self, area: &Rect, emu: &EmuState) -> Widget {
        let uart_lines = emu.devices.get_stat(0).unwrap();
        let uart_lines = uart_lines.lines().collect::<Vec<_>>();
        let view_begin_idx = max(0, (uart_lines.len() as i32) - (area.height as i32) + 2);
        let uart_output = uart_lines[view_begin_idx as usize..].join("\n");

        Widget::default().title(" Device 0: UART ").body(Text::raw(uart_output))
    }

    pub fn handle_key_event(&mut self, _: KeyEvent) {}
}
