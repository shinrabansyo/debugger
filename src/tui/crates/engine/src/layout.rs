pub(crate) mod build;
pub(crate) mod control;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ratatui::layout::Rect;

use crate::widget::Widget;
use build::LayoutBuilder;
use control::{select, map, Direction, LayoutTree, RcLayoutTree};

pub struct Layout {
    tree: RcLayoutTree,
    cursor: RcLayoutTree,
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
        let cursor = LayoutTree::top_widget(&tree);

        (widgets, Layout { tree, cursor })
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        if let Some(cursor) = select(&self.cursor, direction) {
            self.cursor = cursor;
        }
    }

    pub fn get_cursor(&self) -> u8 {
        if let LayoutTree::Widget { id, .. } = &(*self.cursor.borrow()) {
            *id
        } else {
            unreachable!()
        }
    }

    pub fn map(&self, target: Rect) -> HashMap<u8, Rect> {
        map(&self.tree, target)
    }
}
