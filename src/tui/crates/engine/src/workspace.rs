mod emb_status;
mod emb_help;

use std::cmp::{min, max};

use crossterm::event::{KeyCode, KeyEvent};

use sb_emu::State as EmuState;

use crate::layout::Layout;
use crate::widget::{Widget, WidgetView};
use emb_status::Status;
use emb_help::Help;

#[derive(Default)]
pub struct WorkspaceBuilder {
    name: Option<String>,
    widgets: Vec<((i8, i8), Box<dyn Widget>)>,
}

impl WorkspaceBuilder {
    pub fn name(mut self, name: impl Into<String>) -> WorkspaceBuilder {
        self.name = Some(name.into());
        self
    }

    pub fn widget(mut self, pos: (i8, i8), state: Box<dyn Widget>) -> WorkspaceBuilder {
        self.widgets.push((pos, state));
        self
    }

    pub fn build(self) -> Workspace {
        let mut stat_widget = Status::default();
        stat_widget.set_workspace_name(self.name.unwrap_or("Workspace".to_string()));

        Workspace {
            widgets: self.widgets,
            stat_widget,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct Workspace {
    // ユーザ指定ウィジェット
    widgets: Vec<((i8, i8), Box<dyn Widget>)>,

    // 固定で持つウィジェット
    stat_widget: Status,
    help_widget: Help,

    // 全体の状態
    cursor: (i8, i8),
    input_mode: bool,
}

impl Workspace {
    pub fn affect(&self, mut emu: EmuState) -> EmuState {
        for (_, state) in &self.widgets {
            emu = state.affect(emu);
        }
        emu
    }

    pub gen fn draw(&self, layout: &Layout, emu: &EmuState) -> WidgetView {
        /* ==== TODO ==== */
        yield self.widgets[0].1.draw(&layout.inst, emu).selected(self.widgets[0].0 == self.cursor);
        yield self.widgets[1].1.draw(&layout.device, emu).selected(self.widgets[1].0 == self.cursor);
        yield self.widgets[2].1.draw(&layout.state, emu).selected(self.widgets[2].0 == self.cursor);
        yield self.widgets[3].1.draw(&layout.memory, emu).selected(self.widgets[3].0 == self.cursor);
        /* ==== TODO ==== */

        yield self.stat_widget.draw(&layout.mode, emu);
        yield self.help_widget.draw(&layout.help, emu);

        ()
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        if self.input_mode {
            match event.code {
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.stat_widget.set_input_mode(false);
                }
                _ => {
                    for (pos, state) in &mut self.widgets {
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
                    self.stat_widget.set_input_mode(true);
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
