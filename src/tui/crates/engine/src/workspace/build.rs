use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::widget::Widget;
use super::emb_widget::{Help, Status};
use super::layout::build::LayoutBuilder;
use super::layout::Layout;
use super::Workspace;

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
            input_mode: false,
        }
    }
}
