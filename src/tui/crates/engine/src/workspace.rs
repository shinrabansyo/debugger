mod emb_status;
mod emb_help;

use std::cell::RefCell;
use std::cmp::{min, max};
use std::collections::HashMap;
use std::rc::Rc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use sb_emu::Emulator;

use crate::layout::build::LayoutBuilder;
use crate::layout::Layout;
use crate::widget::Widget;
use emb_status::Status;
use emb_help::Help;

#[derive(Default)]
pub struct WorkspaceBuilder {
    name: Option<String>,
    widgets: HashMap<u8, Rc<RefCell<dyn Widget>>>,
    layout: Option<Layout>,
    stat_widget: Option<Rc<RefCell<Status>>>,
}

impl WorkspaceBuilder {
    pub fn name(mut self, name: impl Into<String>) -> WorkspaceBuilder {
        self.name = Some(name.into());
        self
    }

    pub fn layout<F>(mut self, build_fn: F) -> Self
    where
        F: FnOnce(&mut LayoutBuilder),
    {
        self.stat_widget = Some(Rc::new(RefCell::new(Status::default())));

        let (widgets, layout) = Layout::build(|l| {
            l.split_v(100, |l| {
                build_fn(l);
                l.split_h(1, |l| {
                    l.put(20, &Status::upcast(self.stat_widget.as_ref().unwrap()));
                    l.put(80, &Help::new());
                });
            });
        });

        self.widgets = widgets;
        self.layout = Some(layout);
        self
    }

    pub fn build(self) -> Workspace {
        let stat_widget = self.stat_widget.unwrap();
        stat_widget.borrow_mut().set_workspace_name(self.name.unwrap());

        Workspace {
            widgets: self.widgets,
            layout: self.layout.unwrap(),
            stat_widget,
            cursor: (0, 0),
            input_mode: false,
        }
    }
}

pub struct Workspace {
    // UI 配置
    widgets: HashMap<u8, Rc<RefCell<dyn Widget>>>,
    layout: Layout,

    // 固定で持つウィジェット
    stat_widget: Rc<RefCell<Status>>,

    // 全体の状態
    cursor: (i8, i8),
    input_mode: bool,
}

impl Workspace {
    pub fn draw(&self, frame: &mut Frame, emu: &Emulator) {
        for (id, area) in self.layout.map(frame.area()) {
            let widget = self.widgets.get(&id).unwrap();
            let view = widget.borrow().draw(&area, emu);
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
                    // for (pos, widget) in &mut self.widgets {
                    //     if pos == &self.cursor {
                    //         widget.borrow_mut().on_key_pressed(event);
                    //         break;
                    //     }
                    // }
                }
            }
        } else {
            match event.code {
                KeyCode::Char('i') => {
                    self.input_mode = true;
                    self.stat_widget.borrow_mut().set_input_mode(true);
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
