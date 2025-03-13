mod build;
mod emb_widget;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use sb_emu::Emulator;

use crate::layout::control::Direction;
use crate::layout::Layout;
use crate::widget::Widget;
use build::WorkspaceBuilder;
use emb_widget::Status;

pub struct Workspace {
    // UI 配置
    widgets: HashMap<u8, Rc<RefCell<dyn Widget>>>,
    layout: Layout,

    // 固定で持つウィジェット
    stat_widget: Rc<RefCell<Status>>,

    // モード
    input_mode: bool,
}

impl Workspace {
    pub fn builder() -> WorkspaceBuilder {
        WorkspaceBuilder::default()
    }

    pub fn draw(&self, frame: &mut Frame, emu: &Emulator) {
        let cursor = self.layout.get_cursor();
        for (id, area) in self.layout.map(frame.area()) {
            let widget = self.widgets.get(&id).unwrap();
            let view = widget.borrow().draw(&area, emu).selected(cursor == id);
            frame.render_widget(view, area);
        }
    }

    pub fn on_emu_updating(&self, emu: &mut Emulator) {
        for (_, widget) in self.widgets.iter() {
            widget.borrow_mut().on_emu_updating(emu);
        }
    }

    pub fn on_key_pressed(&mut self, event: KeyEvent) {
        if self.input_mode {
            match event.code {
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.stat_widget.borrow_mut().set_input_mode(false);
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
                    self.input_mode = true;
                    self.stat_widget.borrow_mut().set_input_mode(true);
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
