mod build;
mod emb_widget;
mod layout;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use sb_dbg::Debugger;

use crate::widget::Widget;
use build::WorkspaceBuilder;
use emb_widget::Status;
use layout::control::Direction;
use layout::Layout;

pub struct Workspace {
    // UI 配置
    widgets: HashMap<u8, Rc<RefCell<dyn Widget>>>,
    layout: Layout,

    // 固定で持つウィジェット
    stat_widget: Rc<RefCell<Status>>,

    // モード
    control_mode: bool,
}

impl Workspace {
    pub fn builder() -> WorkspaceBuilder {
        WorkspaceBuilder::default()
    }

    pub fn draw(&self, frame: &mut Frame, debugger: &Debugger) {
        let cursor = self.layout.get_cursor();
        for (id, area) in self.layout.map(frame.area()) {
            let widget = self.widgets.get(&id).unwrap();
            let view = widget.borrow().draw(&area, debugger).selected(cursor == id);
            frame.render_widget(view, area);
        }
    }

    pub fn on_debugger_updating(&self, debugger: &mut Debugger) {
        for (_, widget) in self.widgets.iter() {
            widget.borrow_mut().on_debugger_updating(debugger);
        }
    }

    pub fn on_key_pressed(&mut self, event: KeyEvent) {
        if self.control_mode {
            match event.code {
                KeyCode::Esc => {
                    self.control_mode = false;
                    self.stat_widget.borrow_mut().set_control_mode(false);
                }
                _ => {
                    let cursor = self.layout.get_cursor();
                    let widget = self.widgets.get(&cursor).unwrap();
                    widget.borrow_mut().on_key_pressed(event);
                }
            }
        } else {
            match event.code {
                KeyCode::Char('i') => {
                    self.control_mode = true;
                    self.stat_widget.borrow_mut().set_control_mode(true);
                }
                KeyCode::Char('h') => self.layout.move_cursor(Direction::Left),
                KeyCode::Char('l') => self.layout.move_cursor(Direction::Right),
                KeyCode::Char('k') => self.layout.move_cursor(Direction::Up),
                KeyCode::Char('j') => {
                    // 一番下のライン (Stat や Help) に移動する場合は移動を拒否
                    let new_cursor = self.layout.try_move_cursor(Direction::Down);
                    if new_cursor != (self.widgets.len() - 1) as u8 {
                        self.layout.move_cursor(Direction::Down);
                    }
                }
                _ => {}
            }
        }
    }
}
