pub(crate) mod build;
mod control;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ratatui::layout::Rect;

use crate::widget::Widget;
use build::LayoutBuilder;
use control::{map, LayoutTree, RcLayoutTree};

pub struct Layout {
    tree: RcLayoutTree,
}

impl Layout {
    pub fn build<F>(build_fn: F) -> (HashMap<u8, Rc<RefCell<dyn Widget>>>, Layout)
    where
        F: FnOnce(&mut LayoutBuilder),
    {
        let mut builder = LayoutBuilder::new();
        build_fn(&mut builder);

        let (widgets, tree) = builder.build();
        let tree = LayoutTree::wrap(tree);

        (widgets, Layout { tree })
    }

    pub fn map(&self, target: Rect) -> HashMap<u8, Rect> {
        map(&self.tree, target)
    }
}
