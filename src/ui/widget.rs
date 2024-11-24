mod inst_view;
mod outout_view;
mod reg_view;
mod mem_view;

use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};

use sb_emu::State as EmuState;

use inst_view::{InstView, InstViewState};
use outout_view::{OutputView, OutputViewState};
use reg_view::{RegisterView, RegisterViewState};
use mem_view::{MemView, MemViewState};

pub struct Widgets {
    pub inst_view: InstView,
    pub output_view: OutputView,
    pub state_view: RegisterView,
    pub mem_view: MemView,
}

pub struct WidgetsManager {
    // 各 Widget の状態
    inst_view_state: InstViewState,
    output_view_state: OutputViewState,
    state_view_state: RegisterViewState,
    mem_view_state: MemViewState,

    // 全体の状態
    cursor: (i32, i32),
}

impl WidgetsManager {
    pub fn new(emu: &EmuState) -> Self {
        WidgetsManager {
            inst_view_state: InstViewState::new(true, emu),
            output_view_state: OutputViewState::new(false, emu),
            state_view_state: RegisterViewState::new(false, emu),
            mem_view_state: MemViewState::new(false, emu),
            cursor: (0, 0),
        }
    }

    pub fn gen_widgets(&self) -> Widgets {
        Widgets {
            inst_view: self.inst_view_state.gen_widget(),
            output_view: self.output_view_state.gen_widget(),
            state_view: self.state_view_state.gen_widget(),
            mem_view: self.mem_view_state.gen_widget(),
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
                    (0, 0) => self.inst_view_state.handle_key_event(event),
                    (0, 1) => self.output_view_state.handle_key_event(event),
                    (1, 0) => self.state_view_state.handle_key_event(event),
                    (1, 1) => self.mem_view_state.handle_key_event(event),
                    _ => {}
                }
            }
        }

        self.inst_view_state.set_selected(self.cursor == (0, 0));
        self.output_view_state.set_selected(self.cursor == (0, 1));
        self.state_view_state.set_selected(self.cursor == (1, 0));
        self.mem_view_state.set_selected(self.cursor == (1, 1));
    }

    pub fn update_emu(&mut self, emu: &EmuState) {
        self.inst_view_state.update_emu(&emu);
        self.output_view_state.update_emu(&emu);
        self.state_view_state.update_emu(&emu);
        self.mem_view_state.update_emu(&emu);
    }
}
