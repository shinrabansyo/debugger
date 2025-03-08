mod inst;
mod device;
mod reg;
mod mem;
mod mode;
mod help;

use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;

use sb_emu::State as EmuState;

use crate::ui::layout::Layout;
use inst::Inst;
use device::Device;
use reg::Register;
use mem::Mem;
use mode::Mode;
use help::Help;

pub trait Widget {
    type State: WidgetState;
}

pub trait WidgetState
where
    Self: Default,
{
    type Widget: Widget;

    fn affect(&self, emu: EmuState) -> EmuState;
    fn draw(&self, area: &Rect, emu: &EmuState) -> Self::Widget;
    fn handle_key_event(&mut self, event: KeyEvent);
    fn set_selected(&mut self, selected: bool);
}

pub struct Widgets {
    pub inst: Inst,
    pub device: Device,
    pub state: Register,
    pub mem: Mem,
    pub mode: Mode,
    pub help: Help,
}

#[derive(Default)]
pub struct WidgetsManager {
    // 各 Widget の状態
    inst_state: <Inst as Widget>::State,
    device_state: <Device as Widget>::State,
    state_state: <Register as Widget>::State,
    mem_state: <Mem as Widget>::State,
    mode_state: <Mode as Widget>::State,
    help_state: <Help as Widget>::State,

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

    pub fn draw(&self, layout: &Layout, emu: &EmuState) -> Widgets {
        Widgets {
            inst: self.inst_state.draw(&layout.inst, emu),
            device: self.device_state.draw(&layout.device, emu),
            state: self.state_state.draw(&layout.state, emu),
            mem: self.mem_state.draw(&layout.memory, emu),
            mode: self.mode_state.draw(&layout.mode, emu),
            help: self.help_state.draw(&layout.help, emu),
        }
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
