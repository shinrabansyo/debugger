mod inst_view;
mod outout_view;
mod state_view;
mod mem_view;

use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};

use inst_view::{InstView, InstViewState};
use outout_view::{OutputView, OutputViewState};
use state_view::{StateView, StateViewState};
use mem_view::{MemView, MemViewState};

pub struct Widgets {
    pub inst_view: InstView,
    pub output_view: OutputView,
    pub state_view: StateView,
    pub mem_view: MemView,
}

pub struct WidgetsManager {
    // 各 Widget の状態
    inst_view_state: InstViewState,
    output_view_state: OutputViewState,
    state_view_state: StateViewState,
    mem_view_state: MemViewState,

    // 全体の状態
    cursor: (i32, i32),
}

impl WidgetsManager {
    pub fn new() -> Self {
        WidgetsManager {
            inst_view_state: InstViewState::new(true),
            output_view_state: OutputViewState::new(false),
            state_view_state: StateViewState::new(false),
            mem_view_state: MemViewState::new(false),
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
            _ => {}
        }
        self.update_widgets();
    }

    fn update_widgets(&mut self) {
        // cursor
        self.inst_view_state.selected = self.cursor == (0, 0);
        self.output_view_state.selected = self.cursor == (0, 1);
        self.state_view_state.selected = self.cursor == (1, 0);
        self.mem_view_state.selected = self.cursor == (1, 1);
    }
}
