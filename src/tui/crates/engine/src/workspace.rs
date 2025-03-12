mod emb_status;
mod emb_help;

use std::cell::RefCell;
use std::cmp::{min, max};
use std::rc::Rc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use sb_emu::Emulator;

use crate::layout::Layout;
use crate::widget::Widget;
use emb_status::Status;
use emb_help::Help;

#[derive(Default)]
pub struct WorkspaceBuilder {
    name: Option<String>,
    widgets: Vec<((i8, i8), Rc<RefCell<dyn Widget>>)>,
}

impl WorkspaceBuilder {
    pub fn name(mut self, name: impl Into<String>) -> WorkspaceBuilder {
        self.name = Some(name.into());
        self
    }

    pub fn widget(mut self, pos: (i8, i8), state: &Rc<RefCell<dyn Widget>>) -> WorkspaceBuilder {
        self.widgets.push((pos, Rc::clone(state)));
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
    widgets: Vec<((i8, i8), Rc<RefCell<dyn Widget>>)>,

    // 固定で持つウィジェット
    stat_widget: Status,
    help_widget: Help,

    // 全体の状態
    cursor: (i8, i8),
    input_mode: bool,
}

impl Workspace {
    pub fn draw(&self, frame: &mut Frame, layout: &Layout, emu: &Emulator) {
        /* ==== TODO ==== */
        let widget = self.widgets[0].1.borrow();
        let view_inst = widget.draw(&layout.inst, emu).selected(self.widgets[0].0 == self.cursor);
        frame.render_widget(view_inst, layout.inst);

        let widget = self.widgets[1].1.borrow();
        let view_device = widget.draw(&layout.device, emu).selected(self.widgets[1].0 == self.cursor);
        frame.render_widget(view_device, layout.device);

        let widget = self.widgets[2].1.borrow();
        let view_state = widget.draw(&layout.state, emu).selected(self.widgets[2].0 == self.cursor);
        frame.render_widget(view_state, layout.state);

        let widget = self.widgets[3].1.borrow();
        let view_memory = widget.draw(&layout.memory, emu).selected(self.widgets[3].0 == self.cursor);
        frame.render_widget(view_memory, layout.memory);
        /* ==== TODO ==== */

        let view_stat = self.stat_widget.draw(&layout.mode, emu).selected(self.cursor == (-1, -1));
        frame.render_widget(view_stat, layout.mode);

        let view_help = self.help_widget.draw(&layout.help, emu).selected(self.cursor == (-1, -1));
        frame.render_widget(view_help, layout.help);
    }

    pub fn on_emu_updating(&self, emu: &mut Emulator) {
        for (_, widget) in &self.widgets {
            widget.borrow_mut().on_emu_updating(emu);
        }
    }

    pub fn on_key_pressed(&mut self, event: KeyEvent) {
        if self.input_mode {
            match event.code {
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.stat_widget.set_input_mode(false);
                }
                _ => {
                    for (pos, widget) in &mut self.widgets {
                        if pos == &self.cursor {
                            widget.borrow_mut().on_key_pressed(event);
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
