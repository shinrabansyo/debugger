use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget as WidgetR, Block};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::Line;

use sb_emu::State as EmuState;

#[derive(Default)]
pub struct Widget {
    selected: bool,
    title: Option<String>,
    constructor: Option<Box<dyn FnOnce(Rect, &mut Buffer, bool, Option<String>) -> ()>>,
}

impl ratatui::widgets::Widget for Widget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constructor = self.constructor.unwrap();
        constructor(area, buf, self.selected, self.title);
    }
}

impl Widget {
    pub fn selected(mut self, selected: bool) -> Widget {
        self.selected = selected;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Widget {
        self.title = Some(title.into());
        self
    }

    pub fn body(mut self, body: impl ratatui::widgets::Widget + 'static) -> Widget {
        fn make_outer(area: Rect, buf: &mut Buffer, selected: bool, title: Option<String>) -> Rect {
            if let Some(title) = title {
                let title = Line::from(title).bold().centered();
                let border = if selected { border::THICK } else { border::ROUNDED };

                let block = Block::bordered().title(title).border_set(border);
                let area_inner = block.inner(area);
                block.render(area, buf);

                area_inner
            } else {
                area
            }
        }

        self.constructor = Some(Box::new(
            move |area: Rect, buf: &mut Buffer, selected: bool, title: Option<String>| {
                let area = make_outer(area, buf, selected, title);
                body.render(area, buf);
            }));
        self
    }
}

pub trait WidgetState {
    fn draw(&self, area: &Rect, emu: &EmuState) -> Widget;

    fn handle_key_event(&mut self, _: KeyEvent) {}

    fn affect(&self, emu: EmuState) -> EmuState { emu }
}
