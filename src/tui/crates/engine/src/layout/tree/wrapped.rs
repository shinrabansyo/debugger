use std::cell::RefCell;
use std::rc::Rc;

use super::raw::LayoutTree as RawLayoutTree;

pub(crate) type RcLayoutTree = Rc<RefCell<LayoutTree>>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LayoutTree {
    Horizontal {
        size: u16,
        children: Vec<Rc<RefCell<LayoutTree>>>,
    },
    Vertical {
        size: u16,
        children: Vec<Rc<RefCell<LayoutTree>>>,
    },
    Widget {
        id: u8,
        size: u16,
    },
}

impl LayoutTree {
    pub(crate) fn wrap(raw: RawLayoutTree) -> RcLayoutTree {
        let tree = match raw {
            RawLayoutTree::Horizontal { size, children } => {
                let children = children
                    .into_iter()
                    .map(LayoutTree::wrap)
                    .collect();
                LayoutTree::Horizontal { size, children }
            }
            RawLayoutTree::Vertical { size, children } => {
                let children = children
                    .into_iter()
                    .map(LayoutTree::wrap)
                    .collect();
                LayoutTree::Vertical { size, children }
            }
            RawLayoutTree::Widget { id, size } => LayoutTree::Widget { id, size },
        };
        Rc::new(RefCell::new(tree))
    }

    pub(crate) fn size(&self) -> u16 {
        match self {
            LayoutTree::Horizontal { size, .. } => *size,
            LayoutTree::Vertical { size, .. } => *size,
            LayoutTree::Widget { size, .. } => *size,
        }
    }
}
