use std::cell::RefCell;
use std::rc::Rc;

use crate::layout::build::tree::LayoutTree as RawLayoutTree;

pub type RcLayoutTree = Rc<RefCell<LayoutTree>>;

#[derive(Debug, PartialEq, Eq)]
pub enum LayoutTree {
    Horizontal {
        size: u16,
        parent: Option<Rc<RefCell<LayoutTree>>>,
        children: Vec<Rc<RefCell<LayoutTree>>>,
    },
    Vertical {
        size: u16,
        parent: Option<Rc<RefCell<LayoutTree>>>,
        children: Vec<Rc<RefCell<LayoutTree>>>,
    },
    Widget {
        id: u8,
        size: u16,
        parent: Option<Rc<RefCell<LayoutTree>>>,
    },
}

impl LayoutTree {
    pub fn wrap(node: RawLayoutTree) -> RcLayoutTree {
        match node {
            RawLayoutTree::Horizontal { size, children } => {
                // ノード準備 (仮)
                let node = Rc::new(RefCell::new(LayoutTree::Horizontal {
                    size,
                    parent: None,
                    children: vec![],
                }));

                // 子ノードセットアップ
                let children = children
                    .into_iter()
                    .map(LayoutTree::wrap)
                    .collect::<Vec<_>>();
                for child in &children {
                    child.borrow_mut().set_parent(Rc::clone(&node));
                }

                // 現ノードに対して子ノードを設定
                node.borrow_mut().set_children(children);

                node
            }
            RawLayoutTree::Vertical { size, children } => {
                // ノード準備 (仮)
                let node = Rc::new(RefCell::new(LayoutTree::Vertical {
                    size,
                    parent: None,
                    children: vec![],
                }));

                // 子ノードセットアップ
                let children = children
                    .into_iter()
                    .map(LayoutTree::wrap)
                    .collect::<Vec<_>>();
                for child in &children {
                    child.borrow_mut().set_parent(Rc::clone(&node));
                }

                // 現ノードに対して子ノードを設定
                node.borrow_mut().set_children(children);

                node
            }
            RawLayoutTree::Widget { id, size } => {
                let node = LayoutTree::Widget { id, size, parent: None };
                Rc::new(RefCell::new(node))
            }
        }
    }

    fn set_parent(&mut self, parent: RcLayoutTree) {
        match self {
            LayoutTree::Horizontal { parent: p, .. } => *p = Some(parent),
            LayoutTree::Vertical { parent: p, .. } => *p = Some(parent),
            LayoutTree::Widget { parent: p, .. } => *p = Some(parent),
        }
    }

    fn set_children(&mut self, children: Vec<RcLayoutTree>) {
        match self {
            LayoutTree::Horizontal { children: c, .. } => *c = children,
            LayoutTree::Vertical { children: c, .. } => *c = children,
            _ => unreachable!(),
        }
    }

    pub fn top_widget(node: &RcLayoutTree) -> RcLayoutTree {
        match &(*node.borrow()) {
            LayoutTree::Horizontal { children, .. } => {
                let child = children.first().unwrap();
                LayoutTree::top_widget(child)
            }
            LayoutTree::Vertical { children, .. } => {
                let child = children.first().unwrap();
                LayoutTree::top_widget(child)
            }
            LayoutTree::Widget { .. } => Rc::clone(node),
        }
    }

    pub fn parent(&self) -> &Option<RcLayoutTree> {
        match self {
            LayoutTree::Horizontal { parent, .. } => parent,
            LayoutTree::Vertical { parent, .. } => parent,
            LayoutTree::Widget { parent, .. } => parent,
        }
    }

    pub fn size(&self) -> u16 {
        match self {
            LayoutTree::Horizontal { size, .. } => *size,
            LayoutTree::Vertical { size, .. } => *size,
            LayoutTree::Widget { size, .. } => *size,
        }
    }
}
