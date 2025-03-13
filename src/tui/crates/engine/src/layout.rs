mod build;
mod mapping;
mod tree;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ratatui::layout::Rect;

pub(crate) use build::LayoutBuilder;

use crate::widget::Widget;
use mapping::mapping;
use tree::raw::LayoutTree;

pub struct Layout {
    tree: LayoutTree,
}

impl Layout {
    pub fn build<F>(build_fn: F) -> (HashMap<u8, Rc<RefCell<dyn Widget>>>, Layout)
    where
        F: FnOnce(&mut LayoutBuilder),
    {
        let mut builder = LayoutBuilder::new();
        build_fn(&mut builder);
        let (widgets, tree) = builder.build();
        (widgets, Layout { tree })
    }

    pub fn mapping(&self, target: Rect) -> HashMap<u8, Rect> {
        mapping(&self.tree, target)
    }
}
