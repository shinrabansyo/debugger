mod inst;
mod device;
mod reg;
mod mem;
mod help;

use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};

use sb_emu::State as EmuState;

use inst::{Inst, InstState};
use device::{Device, DeviceState};
use reg::{Register, RegisterState};
use mem::{Mem, MemState};
use help::Help;

pub struct Widgets {
    pub inst: Inst,
    pub device: Device,
    pub state: Register,
    pub mem: Mem,
    pub help: Help,
}

pub struct WidgetsManager {
    // 各 Widget の状態
    inst_state: InstState,
    device_state: DeviceState,
    state_state: RegisterState,
    mem_state: MemState,

    // 全体の状態
    cursor: (i32, i32),
}

impl WidgetsManager {
    pub fn new() -> Self {
        WidgetsManager {
            inst_state: InstState::new(true),
            device_state: DeviceState::new(false),
            state_state: RegisterState::new(false),
            mem_state: MemState::new(false),
            cursor: (0, 0),
        }
    }

    pub fn gen_widgets(&self, emu: &EmuState) -> Widgets {
        Widgets {
            inst: self.inst_state.gen_widget(emu),
            device: self.device_state.gen_widget(emu),
            state: self.state_state.gen_widget(emu),
            mem: self.mem_state.gen_widget(emu),
            help: Help,
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('h') => self.cursor.0 = max(0, self.cursor.0 - 1),
            KeyCode::Char('l') => self.cursor.0 = min(1, self.cursor.0 + 1),
            KeyCode::Char('k') => self.cursor.1 = max(0, self.cursor.1 - 1),
            KeyCode::Char('j') => self.cursor.1 = min(1, self.cursor.1 + 1),
            _ => {
                match self.cursor {
                    (0, 0) => self.inst_state.handle_key_event(event),
                    (0, 1) => self.device_state.handle_key_event(event),
                    (1, 0) => self.state_state.handle_key_event(event),
                    (1, 1) => self.mem_state.handle_key_event(event),
                    _ => {}
                }
            }
        }

        self.inst_state.set_selected(self.cursor == (0, 0));
        self.device_state.set_selected(self.cursor == (0, 1));
        self.state_state.set_selected(self.cursor == (1, 0));
        self.mem_state.set_selected(self.cursor == (1, 1));
    }
}
