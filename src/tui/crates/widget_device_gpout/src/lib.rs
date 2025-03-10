use ratatui::layout::Rect;
use ratatui::text::Text;

use sb_emu::State as EmuState;
use sb_dbg_tui_engine::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Gpout;

impl Widget for Gpout {
    fn draw(&self, _: &Rect, emu: &EmuState) -> WidgetView {
        WidgetView::default()
            .title(" Device: GPIO (Out) ")
            .body(Text::raw(emu.devices.get_stat(4).unwrap()))
    }
}
