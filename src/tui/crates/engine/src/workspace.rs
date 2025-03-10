mod emb_mode;
mod emb_help;

use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};

use sb_emu::State as EmuState;

use crate::layout::Layout;
use crate::widget::{Widget, WidgetState};
use emb_mode::ModeState;
use emb_help::HelpState;

#[derive(Default)]
pub struct WorkspaceBuilder {
    name: Option<String>,
    states: Vec<((i8, i8), Box<dyn WidgetState>)>,
}

impl WorkspaceBuilder {
    pub fn name(mut self, name: impl Into<String>) -> WorkspaceBuilder {
        self.name = Some(name.into());
        self
    }

    pub fn widget(mut self, pos: (i8, i8), state: Box<dyn WidgetState>) -> WorkspaceBuilder {
        self.states.push((pos, state));
        self
    }

    pub fn build(self) -> Workspace {
        let mut mode_state = ModeState::default();
        mode_state.set_workspace_name(self.name.unwrap_or("Workspace".to_string()));

        Workspace {
            states: self.states,
            mode_state: mode_state,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct Workspace {
    // ユーザ指定ウィジェット
    states: Vec<((i8, i8), Box<dyn WidgetState>)>,

    // 固定で持つウィジェット
    mode_state: ModeState,
    help_state: HelpState,

    // 全体の状態
    cursor: (i8, i8),
    input_mode: bool,
}

impl Workspace {
    pub fn affect(&self, mut emu: EmuState) -> EmuState {
        for (_, state) in &self.states {
            emu = state.affect(emu);
        }
        emu
    }

    pub gen fn draw(&self, layout: &Layout, emu: &EmuState) -> Widget {
        for (pos, state) in &self.states {
            yield state.draw(&layout.inst, emu).selected(pos == &self.cursor);
        }
        yield self.mode_state.draw(&layout.mode, emu);
        yield self.help_state.draw(&layout.help, emu);
        ()
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        if self.input_mode {
            match event.code {
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.mode_state.set_input_mode(false);
                }
                _ => {
                    for (pos, state) in &mut self.states {
                        if pos == &self.cursor {
                            state.handle_key_event(event);
                            break;
                        }
                    }
                }
            }
        } else {
            match event.code {
                KeyCode::Char('i') => {
                    self.input_mode = true;
                    self.mode_state.set_input_mode(true);
                }
                KeyCode::Char('h') => self.cursor.0 = max(0, self.cursor.0 - 1),
                KeyCode::Char('l') => self.cursor.0 = min(1, self.cursor.0 + 1),
                KeyCode::Char('k') => self.cursor.1 = max(0, self.cursor.1 - 1),
                KeyCode::Char('j') => self.cursor.1 = min(1, self.cursor.1 + 1),
                _ => {}
            }
        }
    }
}
