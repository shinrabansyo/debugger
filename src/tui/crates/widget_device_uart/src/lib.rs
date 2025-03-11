use std::cmp::max;

use ratatui::layout::Rect;
use ratatui::text::Text;

use sb_emu::State as EmuState;
use sb_dbg_tui_engine::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Uart;

impl Widget for Uart {
    fn draw(&self, area: &Rect, emu: &EmuState) -> WidgetView {
        let uart_lines = emu.devices.get_uart_stat();
        let uart_lines = uart_lines.lines().collect::<Vec<_>>();
        let view_begin_idx = max(0, (uart_lines.len() as i32) - (area.height as i32) + 2);
        let uart_output = uart_lines[view_begin_idx as usize..].join("\n");

        WidgetView::default().title(" Device: UART ").body(Text::raw(uart_output))
    }
}
