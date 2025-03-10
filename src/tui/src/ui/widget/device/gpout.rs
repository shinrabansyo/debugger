use ratatui::text::Text;
use crossterm::event::KeyEvent;

use sb_emu::State as EmuState;

use crate::ui::widget::Widget;

#[derive(Default)]
pub struct GPOut;

impl GPOut {
    pub fn draw(&self, emu: &EmuState) -> Widget {
        Widget::default()
            .selected(false)
            .title(" Device 1: GPIO (Out) ")
            .body(Text::raw(emu.devices.get_stat(4).unwrap()))
    }

    pub fn handle_key_event(&mut self, _: KeyEvent) {}
}
