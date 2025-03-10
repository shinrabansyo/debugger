use ratatui::layout::Rect;
use ratatui::text::Text;

use sb_emu::State as EmuState;
use sb_dbg_tui_engine::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Display;

impl Widget for Display {
    fn draw(&self, _: &Rect, _: &EmuState) -> WidgetView {
        WidgetView::default().title(" Device: Display ").body(Text::raw(""))
    }
}
