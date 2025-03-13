use std::collections::HashMap;

use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::layout::Layout as RataLayout;

use super::tree::{LayoutTree, RcLayoutTree};

pub(crate) fn map(tree: &RcLayoutTree, target: Rect) -> HashMap<u8, Rect> {
    let mut driver = MappingDriver::default();
    driver.mapping(tree, target);
    driver.result
}

#[derive(Default)]
struct MappingDriver {
    result: HashMap<u8, Rect>,
}

impl MappingDriver {
    fn mapping(&mut self, node: &RcLayoutTree, target: Rect) {
        match &(*node.borrow()) {
            LayoutTree::Vertical { children, .. } => {
                let constraints = children
                    .iter()
                    .map(|child| Constraint::Percentage(child.borrow().size()))
                    .collect::<Vec<_>>();
                let layout = RataLayout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints)
                    .split(target);
                for (area, child) in layout.iter().zip(children) {
                    self.mapping(child, *area);
                }
            }
            LayoutTree::Horizontal { children, .. } => {
                let constraints = children
                    .iter()
                    .map(|child| Constraint::Percentage(child.borrow().size()))
                    .collect::<Vec<_>>();
                let layout = RataLayout::default()
                    .direction(Direction::Horizontal)
                    .constraints(constraints)
                    .split(target);
                for (area, child) in layout.iter().zip(children) {
                    self.mapping(child, *area);
                }
            }
            LayoutTree::Widget { id, .. } => {
                self.result.insert(*id, target);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use crate::layout::build::tree::LayoutTree as RawLayoutTree;
    use crate::layout::control::tree::LayoutTree;
    use super::map;

    #[test]
    fn map_1() {
        let tree = RawLayoutTree::Horizontal {
            size: 100,
            children: vec![
                RawLayoutTree::Vertical {
                    size: 50,
                    children: vec![
                        RawLayoutTree::Widget { id: 1, size: 50 },
                        RawLayoutTree::Widget { id: 2, size: 50 },
                    ],
                },
                RawLayoutTree::Vertical {
                    size: 50,
                    children: vec![
                        RawLayoutTree::Widget { id: 3, size: 50 },
                        RawLayoutTree::Widget { id: 4, size: 50 },
                    ],
                },
            ],
        };
        let tree = LayoutTree::wrap(tree);

        let target = Rect { x: 0, y: 0, width: 100, height: 100 };
        let layout = map(&tree, target);

        assert_eq!(layout.get(&1), Some(&Rect { x: 0, y: 0, width: 50, height: 50 }));
        assert_eq!(layout.get(&2), Some(&Rect { x: 0, y: 50, width: 50, height: 50 }));
        assert_eq!(layout.get(&3), Some(&Rect { x: 50, y: 0, width: 50, height: 50 }));
        assert_eq!(layout.get(&4), Some(&Rect { x: 50, y: 50, width: 50, height: 50 }));
    }

    #[test]
    fn map_2() {
        let tree = RawLayoutTree::Horizontal {
            size: 100,
            children: vec![
                RawLayoutTree::Vertical {
                    size: 50,
                    children: vec![
                        RawLayoutTree::Widget { id: 1, size: 50 },
                        RawLayoutTree::Widget { id: 2, size: 50 },
                    ],
                },
                RawLayoutTree::Vertical {
                    size: 50,
                    children: vec![
                        RawLayoutTree::Horizontal {
                            size: 50,
                            children: vec![
                                RawLayoutTree::Widget { id: 3, size: 50 },
                                RawLayoutTree::Widget { id: 4, size: 50 },
                            ],
                        },
                        RawLayoutTree::Widget { id: 5, size: 50 },
                    ],
                },
            ],
        };
        let tree = LayoutTree::wrap(tree);

        let target = Rect { x: 0, y: 0, width: 100, height: 100 };
        let layout = map(&tree, target);

        assert_eq!(layout.get(&1), Some(&Rect { x: 0, y: 0, width: 50, height: 50 }));
        assert_eq!(layout.get(&2), Some(&Rect { x: 0, y: 50, width: 50, height: 50 }));
        assert_eq!(layout.get(&3), Some(&Rect { x: 50, y: 0, width: 25, height: 50 }));
        assert_eq!(layout.get(&4), Some(&Rect { x: 75, y: 0, width: 25, height: 50 }));
        assert_eq!(layout.get(&5), Some(&Rect { x: 50, y: 50, width: 50, height: 50 }));
    }

    #[test]
    fn map_3() {
        let tree = RawLayoutTree::Widget { id: 1, size: 100 };
        let tree = LayoutTree::wrap(tree);

        let target = Rect { x: 0, y: 0, width: 100, height: 100 };
        let layout = map(&tree, target);

        assert_eq!(layout.get(&1), Some(&target));
    }

    #[test]
    fn map_4() {
        let tree = RawLayoutTree::Horizontal {
            size: 100,
            children: vec![
                RawLayoutTree::Widget { id: 1, size: 50 },
                RawLayoutTree::Horizontal {
                    size: 50,
                    children: vec![
                        RawLayoutTree::Widget { id: 2, size: 50 },
                        RawLayoutTree::Horizontal {
                            size: 50,
                            children: vec![
                                RawLayoutTree::Widget { id: 3, size: 50 },
                                RawLayoutTree::Horizontal {
                                    size: 50,
                                    children: vec![
                                        RawLayoutTree::Widget { id: 4, size: 50 },
                                        RawLayoutTree::Horizontal {
                                            size: 50,
                                            children: vec![
                                                RawLayoutTree::Widget { id: 5, size: 50 },
                                                RawLayoutTree::Widget { id: 6, size: 50 },
                                            ],
                                        }
                                    ]
                                },
                            ],
                        },
                    ],
                },
            ],
        };
        let tree = LayoutTree::wrap(tree);

        let target = Rect { x: 0, y: 0, width: 100, height: 100 };
        let layout = map(&tree, target);

        assert_eq!(layout.get(&1), Some(&Rect { x: 0, y: 0, width: 50, height: 100 }));
        assert_eq!(layout.get(&2), Some(&Rect { x: 50, y: 0, width: 25, height: 100 }));
        assert_eq!(layout.get(&3), Some(&Rect { x: 75, y: 0, width: 13, height: 100 }));
        assert_eq!(layout.get(&4), Some(&Rect { x: 88, y: 0, width: 6, height: 100 }));
        assert_eq!(layout.get(&5), Some(&Rect { x: 94, y: 0, width: 3, height: 100 }));
        assert_eq!(layout.get(&6), Some(&Rect { x: 97, y: 0, width: 3, height: 100 }));
    }
}
