use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::layout::Layout as RataLayout;

use crate::widget::Widget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SplitDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum LayoutTree {
    Horizontal {
        size: u16,
        children: Vec<LayoutTree>,
    },
    Vertical {
        size: u16,
        children: Vec<LayoutTree>,
    },
    Widget {
        id: u8,
        size: u16,
    },
}

impl LayoutTree {
    fn size(&self) -> u16 {
        match self {
            LayoutTree::Horizontal { size, .. } => *size,
            LayoutTree::Vertical { size, .. } => *size,
            LayoutTree::Widget { size, .. } => *size,
        }
    }
}

pub struct LayoutBuilder {
    // 構築状態
    issued_id: u8,
    split_to: SplitDirection,

    // 結果
    nodes_stack: Vec<LayoutTree>,
    widgets: HashMap<u8, Rc<RefCell<dyn Widget>>>,
}

impl LayoutBuilder {
    pub fn new() -> LayoutBuilder {
        LayoutBuilder {
            issued_id: 0,
            split_to: SplitDirection::Horizontal,
            nodes_stack: vec![],
            widgets: HashMap::new(),
        }
    }

    pub fn split_h<F>(&mut self, size: u16, split: F)
    where
        F: FnOnce(&mut LayoutBuilder),
    {
        let old_stack_len = self.nodes_stack.len();
        let old_split_to = self.split_to;

        // 分割操作
        self.split_to = SplitDirection::Horizontal;
        split(self);

        // 木の構築
        let children = self.nodes_stack.split_off(old_stack_len);
        self.nodes_stack.push(LayoutTree::Horizontal { size, children });

        self.split_to = old_split_to;
    }

    pub fn split_v<F>(&mut self, size: u16, split: F)
    where
        F: FnOnce(&mut LayoutBuilder),
    {
        let old_stack_len = self.nodes_stack.len();
        let old_split_to = self.split_to;

        // 分割操作
        self.split_to = SplitDirection::Vertical;
        split(self);

        // 木の構築
        let children = self.nodes_stack.split_off(old_stack_len);
        self.nodes_stack.push(LayoutTree::Vertical { size, children });

        self.split_to = old_split_to;
    }

    pub fn put(&mut self, size: u16, widget: &Rc<RefCell<dyn Widget>>) {
        self.issued_id += 1;
        self.nodes_stack.push(LayoutTree::Widget {
            id: self.issued_id,
            size,
        });
        self.widgets.insert(self.issued_id, Rc::clone(widget));
    }

    pub fn build(mut self) -> Layout {
        Layout {
            tree: self.nodes_stack.pop().unwrap(),
            widgets: self.widgets,
        }
    }
}

pub struct Layout {
    tree: LayoutTree,
    pub(crate) widgets: HashMap<u8, Rc<RefCell<dyn Widget>>>,
}

impl Layout {
    pub fn mapping(&self, target: Rect) -> Vec<(u8, Rect, Rc<RefCell<dyn Widget>>)> {
        let mut result = vec![];
        self.mapping_inner(&mut result, &self.tree, target);
        result
    }

    fn mapping_inner(
        &self,
        result: &mut Vec<(u8, Rect, Rc<RefCell<dyn Widget>>)>,
        node: &LayoutTree,
        target: Rect
    ) {
        match node {
            LayoutTree::Vertical { children, .. } => {
                let constraints = children
                    .iter()
                    .map(|child| Constraint::Percentage(child.size()))
                    .collect::<Vec<_>>();
                let layout = RataLayout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints)
                    .split(target);
                for (area, child) in layout.iter().zip(children) {
                    self.mapping_inner(result, child, *area);
                }
            }
            LayoutTree::Horizontal { children, .. } => {
                let constraints = children
                    .iter()
                    .map(|child| Constraint::Percentage(child.size()))
                    .collect::<Vec<_>>();
                let layout = RataLayout::default()
                    .direction(Direction::Horizontal)
                    .constraints(constraints)
                    .split(target);
                for (area, child) in layout.iter().zip(children) {
                    self.mapping_inner(result, child, *area);
                }
            }
            LayoutTree::Widget { id, .. }=> {
                result.push((
                    *id,
                    target,
                    Rc::clone(self.widgets.get(id).unwrap())
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use crate::widget::{Widget, WidgetView};

    use super::LayoutBuilder;

    #[derive(Default)]
    struct TestWidget<const ID: usize>;

    impl<const ID: usize> Widget for TestWidget<ID> {
        fn draw(&self, _: &super::Rect, _: &crate::Emulator) -> WidgetView {
            unimplemented!()
        }
    }

    #[test]
    fn layout_build_1() {
        let mut builder = LayoutBuilder::new();
        builder.split_h(100, |l| {
            l.split_v(50, |l| {
                l.put(50, &TestWidget::<0>::new());     // (0, 0)
                l.put(50, &TestWidget::<1>::new());     // (0, 1)
            });
            l.split_v(50, |l| {
                l.put(50, &TestWidget::<2>::new());     // (1, 0)
                l.put(50, &TestWidget::<3>::new());     // (1, 1)
            });
        });

        let target = Rect { x: 0, y: 0, width: 100, height: 100 };
        let layout = builder.build().mapping(target);

        assert_eq!(layout[0].1, Rect { x: 0, y: 0, width: 50, height: 50 });
        assert_eq!(layout[1].1, Rect { x: 0, y: 50, width: 50, height: 50 });
        assert_eq!(layout[2].1, Rect { x: 50, y: 0, width: 50, height: 50 });
        assert_eq!(layout[3].1, Rect { x: 50, y: 50, width: 50, height: 50 });
    }

    #[test]
    fn layout_build_2() {
        let mut builder = LayoutBuilder::new();
        builder.split_h(100, |l| {
            l.split_v(50, |l| {
                l.put(50, &TestWidget::<0>::new());     // (0, 0)
                l.put(50, &TestWidget::<1>::new());     // (0, 1)
            });
            l.split_v(50, |l| {
                l.split_h(50, |l| {
                    l.put(50, &TestWidget::<2>::new()); // (1, 0)
                    l.put(50, &TestWidget::<3>::new()); // (2, 0)
                });
                l.put(50, &TestWidget::<4>::new());     // (1, 1)
            });
        });

        let target = Rect { x: 0, y: 0, width: 100, height: 100 };
        let layout = builder.build().mapping(target);

        assert_eq!(layout[0].1, Rect { x: 0, y: 0, width: 50, height: 50 });
        assert_eq!(layout[1].1, Rect { x: 0, y: 50, width: 50, height: 50 });
        assert_eq!(layout[2].1, Rect { x: 50, y: 0, width: 25, height: 50 });
        assert_eq!(layout[3].1, Rect { x: 75, y: 0, width: 25, height: 50 });
        assert_eq!(layout[4].1, Rect { x: 50, y: 50, width: 50, height: 50 });
    }

    #[test]
    fn layout_build_3() {
        let mut builder = LayoutBuilder::new();
        builder.put(100, &TestWidget::<0>::new());

        let target = Rect { x: 0, y: 0, width: 100, height: 100 };
        let layout = builder.build().mapping(target);

        assert_eq!(layout[0].1, Rect { x: 0, y: 0, width: 100, height: 100 });
    }

    #[test]
    fn layout_build_4() {
        let mut builder = LayoutBuilder::new();
        builder.split_h(100, |l| {
            l.put(50, &TestWidget::<0>::new());                 // (0, 0)
            l.split_h(50, |l| {
                l.put(50, &TestWidget::<1>::new());             // (1, 0)
                l.split_h(50, |l| {
                    l.put(50, &TestWidget::<2>::new());         // (2, 0)
                    l.split_h(50, |l| {
                        l.put(50, &TestWidget::<3>::new());     // (3, 0)
                        l.split_h(50, |l| {
                            l.put(50, &TestWidget::<4>::new()); // (4, 0)
                            l.put(50, &TestWidget::<5>::new()); // (5, 0)
                        });
                    });
                });
            });
        });

        let target = Rect { x: 0, y: 0, width: 100, height: 100 };
        let layout = builder.build().mapping(target);

        assert_eq!(layout[0].1, Rect { x: 0, y: 0, width: 50, height: 100 });
        assert_eq!(layout[1].1, Rect { x: 50, y: 0, width: 25, height: 100 });
        assert_eq!(layout[2].1, Rect { x: 75, y: 0, width: 13, height: 100 });
        assert_eq!(layout[3].1, Rect { x: 88, y: 0, width: 6, height: 100 });
        assert_eq!(layout[4].1, Rect { x: 94, y: 0, width: 3, height: 100 });
        assert_eq!(layout[5].1, Rect { x: 97, y: 0, width: 3, height: 100 });
    }
}
