mod build;
mod mapping;
mod tree;

use std::collections::HashMap;

use ratatui::layout::Rect;

pub use build::LayoutBuilder;
use mapping::mapping;
use tree::raw::LayoutTree;

pub struct Layout {
    tree: LayoutTree,
}

impl Layout {
    pub fn mapping(&self, target: Rect) -> HashMap<u8, Rect> {
        mapping(&self.tree, target)
    }
}
