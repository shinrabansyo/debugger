pub(crate) mod tree;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::widget::Widget;
use tree::LayoutTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SplitDirection {
    Horizontal,
    Vertical,
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

    pub(crate) fn build(mut self) -> (HashMap<u8, Rc<RefCell<dyn Widget>>>, LayoutTree) {
        (self.widgets, self.nodes_stack.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use sb_dbg::Debugger;

    use crate::widget::{Widget, WidgetView};
    use super::{LayoutBuilder, LayoutTree};

    #[derive(Default)]
    struct TestWidget<const ID: usize>;

    impl<const ID: usize> Widget for TestWidget<ID> {
        fn draw(&self, _: &Rect, _: &Debugger) -> WidgetView {
            unimplemented!()
        }
    }

    #[test]
    fn build_1() {
        let mut builder = LayoutBuilder::new();
        builder.split_h(100, |l| {
            l.split_v(50, |l| {
                l.put(50, &TestWidget::<1>::new());     // (0, 0)
                l.put(50, &TestWidget::<2>::new());     // (0, 1)
            });
            l.split_v(50, |l| {
                l.put(50, &TestWidget::<3>::new());     // (1, 0)
                l.put(50, &TestWidget::<4>::new());     // (1, 1)
            });
        });

        assert_eq!(
            builder.nodes_stack.pop().unwrap(),
            LayoutTree::Horizontal {
                size: 100,
                children: vec![
                    LayoutTree::Vertical {
                        size: 50,
                        children: vec![
                            LayoutTree::Widget { id: 1, size: 50 },
                            LayoutTree::Widget { id: 2, size: 50 },
                        ],
                    },
                    LayoutTree::Vertical {
                        size: 50,
                        children: vec![
                            LayoutTree::Widget { id: 3, size: 50 },
                            LayoutTree::Widget { id: 4, size: 50 },
                        ],
                    },
                ],
            },
        );
    }

    #[test]
    fn build_2() {
        let mut builder = LayoutBuilder::new();
        builder.split_h(100, |l| {
            l.split_v(50, |l| {
                l.put(50, &TestWidget::<1>::new());     // (0, 0)
                l.put(50, &TestWidget::<2>::new());     // (0, 1)
            });
            l.split_v(50, |l| {
                l.split_h(50, |l| {
                    l.put(50, &TestWidget::<3>::new()); // (1, 0)
                    l.put(50, &TestWidget::<4>::new()); // (2, 0)
                });
                l.put(50, &TestWidget::<5>::new());     // (1, 1)
            });
        });

        assert_eq!(
            builder.nodes_stack.pop().unwrap(),
            LayoutTree::Horizontal {
                size: 100,
                children: vec![
                    LayoutTree::Vertical {
                        size: 50,
                        children: vec![
                            LayoutTree::Widget { id: 1, size: 50 },
                            LayoutTree::Widget { id: 2, size: 50 },
                        ],
                    },
                    LayoutTree::Vertical {
                        size: 50,
                        children: vec![
                            LayoutTree::Horizontal {
                                size: 50,
                                children: vec![
                                    LayoutTree::Widget { id: 3, size: 50 },
                                    LayoutTree::Widget { id: 4, size: 50 },
                                ],
                            },
                            LayoutTree::Widget { id: 5, size: 50 },
                        ],
                    },
                ],
            },
        );
    }

    #[test]
    fn build_3() {
        let mut builder = LayoutBuilder::new();
        builder.put(100, &TestWidget::<1>::new());

        assert_eq!(
            builder.nodes_stack.pop().unwrap(),
            LayoutTree::Widget { id: 1, size: 100 },
        );
    }

    #[test]
    fn build_4() {
        let mut builder = LayoutBuilder::new();
        builder.split_h(100, |l| {
            l.put(50, &TestWidget::<1>::new());                 // (0, 0)
            l.split_h(50, |l| {
                l.put(50, &TestWidget::<2>::new());             // (1, 0)
                l.split_h(50, |l| {
                    l.put(50, &TestWidget::<3>::new());         // (2, 0)
                    l.split_h(50, |l| {
                        l.put(50, &TestWidget::<4>::new());     // (3, 0)
                        l.split_h(50, |l| {
                            l.put(50, &TestWidget::<5>::new()); // (4, 0)
                            l.put(50, &TestWidget::<6>::new()); // (5, 0)
                        });
                    });
                });
            });
        });

        assert_eq!(
            builder.nodes_stack.pop().unwrap(),
            LayoutTree::Horizontal {
                size: 100,
                children: vec![
                    LayoutTree::Widget { id: 1, size: 50 },
                    LayoutTree::Horizontal {
                        size: 50,
                        children: vec![
                            LayoutTree::Widget { id: 2, size: 50 },
                            LayoutTree::Horizontal {
                                size: 50,
                                children: vec![
                                    LayoutTree::Widget { id: 3, size: 50 },
                                    LayoutTree::Horizontal {
                                        size: 50,
                                        children: vec![
                                            LayoutTree::Widget { id: 4, size: 50 },
                                            LayoutTree::Horizontal {
                                                size: 50,
                                                children: vec![
                                                    LayoutTree::Widget { id: 5, size: 50 },
                                                    LayoutTree::Widget { id: 6, size: 50 },
                                                ],
                                            }
                                        ]
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        );
    }
}
