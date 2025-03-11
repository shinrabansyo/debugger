use std::fmt::Write;

use ratatui::layout::Rect;
use ratatui::text::Text;

use sb_emu::State as EmuState;
use sb_dbg_tui_engine::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Gpout;

impl Widget for Gpout {
    fn draw(&self, _: &Rect, emu: &EmuState) -> WidgetView {
        let gpio_stat= emu.devices.get_gpio_stat();

        let mut line_1 = String::new();
        let mut line_2 = String::new();
        for idx in 0..8 {
            let pin = 7 - idx;
            write!(line_1, " [{}] ", pin).unwrap();
            if gpio_stat & (1 << pin) != 0 {
                write!(line_2, "  O  ").unwrap();
            } else {
                write!(line_2, "  _  ").unwrap();
            }
        }
        let line = format!("{}\n{}", line_1, line_2);

        WidgetView::default().title(" Device: GPIO (Out) ").body(Text::raw(line))
    }
}
