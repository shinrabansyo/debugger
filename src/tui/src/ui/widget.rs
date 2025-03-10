mod inst;
mod device;
mod reg;
mod mem;
mod mode;
mod help;

use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget as WidgetR, Block};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::Line;

use sb_emu::State as EmuState;

use crate::ui::layout::Layout;
use inst::InstState;
use device::DeviceState;
use reg::RegisterState;
use mem::MemState;
use mode::ModeState;
use help::HelpState;

#[derive(Default)]
pub struct Widget {
    selected: bool,
    title: Option<String>,
    constructor: Option<Box<dyn FnOnce(Rect, &mut Buffer) -> ()>>,
}

impl ratatui::widgets::Widget for Widget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constructor = self.constructor.unwrap();
        constructor(area, buf);
    }
}

impl Widget {
    pub fn selected(self, selected: bool) -> Widget {
        Widget {
            selected,
            ..self
        }
    }

    pub fn title(self, title: impl Into<String>) -> Widget {
        Widget {
            title: Some(title.into()),
            ..self
        }
    }

    pub fn body(self, body: impl ratatui::widgets::Widget + 'static) -> Widget {
        let constructor = move |area: Rect, buf: &mut Buffer| {
            // 外枠描画
            let area_inner = if let Some(title) = self.title {
                let title = Line::from(title).bold().centered();
                let border = if self.selected { border::THICK } else { border::ROUNDED };

                let block = Block::bordered().title(title).border_set(border);
                let area_inner = block.inner(area);
                block.render(area, buf);

                area_inner
            } else {
                area
            };

            // 本体描画
            body.render(area_inner, buf);
        };

        Widget {
            title: None,
            constructor: Some(Box::new(constructor)),
            ..self
        }
    }
}

pub trait WidgetState
where
    Self: Default,
{
    fn draw(&self, area: &Rect, emu: &EmuState) -> Widget;
    fn handle_key_event(&mut self, event: KeyEvent);
    fn set_selected(&mut self, selected: bool);

    fn affect(&self, emu: EmuState) -> EmuState {
        emu
    }
}

#[derive(Default)]
pub struct WidgetsManager {
    // 各 Widget の状態
    inst_state: InstState,
    device_state: DeviceState,
    state_state: RegisterState,
    mem_state: MemState,
    mode_state: ModeState,
    help_state: HelpState,

    // 全体の状態
    cursor: (i32, i32),
    input_mode: bool,
}

impl WidgetsManager {
    pub fn affect(&self, emu: EmuState) -> EmuState {
        let emu = self.inst_state.affect(emu);
        let emu = self.device_state.affect(emu);
        let emu = self.state_state.affect(emu);
        let emu = self.mem_state.affect(emu);
        let emu = self.help_state.affect(emu);
        emu
    }

    pub fn draw(&self, layout: &Layout, emu: &EmuState) -> Vec<Widget> {
        vec![
            self.inst_state.draw(&layout.inst, emu),
            self.device_state.draw(&layout.device, emu),
            self.state_state.draw(&layout.state, emu),
            self.mem_state.draw(&layout.memory, emu),
            self.mode_state.draw(&layout.mode, emu),
            self.help_state.draw(&layout.help, emu),
        ]
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        if self.input_mode {
            match event.code {
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.mode_state.handle_key_event(event);
                }
                _ => match self.cursor {
                    (0, 0) => self.inst_state.handle_key_event(event),
                    (0, 1) => self.device_state.handle_key_event(event),
                    (1, 0) => self.state_state.handle_key_event(event),
                    (1, 1) => self.mem_state.handle_key_event(event),
                    _ => {}
                },
            }
        } else {
            match event.code {
                KeyCode::Char('i') => {
                    self.input_mode = true;
                    self.mode_state.handle_key_event(event);
                }
                KeyCode::Char('h') => self.cursor.0 = max(0, self.cursor.0 - 1),
                KeyCode::Char('l') => self.cursor.0 = min(1, self.cursor.0 + 1),
                KeyCode::Char('k') => self.cursor.1 = max(0, self.cursor.1 - 1),
                KeyCode::Char('j') => self.cursor.1 = min(1, self.cursor.1 + 1),
                _ => {}
            }
        }

        self.inst_state.set_selected(self.cursor == (0, 0));
        self.device_state.set_selected(self.cursor == (0, 1));
        self.state_state.set_selected(self.cursor == (1, 0));
        self.mem_state.set_selected(self.cursor == (1, 1));
    }
}
