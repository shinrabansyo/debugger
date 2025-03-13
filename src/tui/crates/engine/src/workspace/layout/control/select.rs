use std::rc::Rc;

use super::tree::{LayoutTree, RcLayoutTree};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn select(node: &RcLayoutTree, direction: Direction) -> Option<RcLayoutTree> {
    select_to_upper(node, direction)
}

fn select_to_upper(node: &RcLayoutTree, direction: Direction) -> Option<RcLayoutTree> {
    // 親ノードを取得
    let node_ref = node.borrow();
    let parent = match node_ref.parent() {
        Some(parent) => parent,
        None => return None,
    };

    // Horizontal に属していて，かつ左右に移動する場合
    if let LayoutTree::Horizontal { children, .. } = &(*parent.borrow()) {
        let at_idx = children
            .iter()
            .position(|c| node == c)
            .unwrap();

        // 移動方向が左，かつ左端にいない場合
        if direction == Direction::Left && at_idx > 0 {
            return select_to_lower(&children[at_idx - 1], direction);
        }

        // 移動方向が右，かつ右端にいない場合
        if direction == Direction::Right && at_idx < children.len() - 1 {
            return select_to_lower(&children[at_idx + 1], direction);
        }
    }

    // Vertical に属していて、かつ、上下に移動する場合
    if let LayoutTree::Vertical { children, .. } = &(*parent.borrow()) {
        let at_idx = children
            .iter()
            .position(|c| node == c)
            .unwrap();

        // 移動方向が上，かつ上端にいない場合
        if direction == Direction::Up && at_idx > 0 {
            return select_to_lower(&children[at_idx - 1], direction);
        }

        // 移動方向が下，かつ下端にいない場合
        if direction == Direction::Down && at_idx < children.len() - 1 {
            return select_to_lower(&children[at_idx + 1], direction);
        }
    }

    // 条件を満たさない場合は親ノードへ移動
    select_to_upper(parent, direction)
}

fn select_to_lower(node: &RcLayoutTree, direction: Direction) -> Option<RcLayoutTree> {
    // Widget だった場合
    if let LayoutTree::Widget { .. } = &(*node.borrow()) {
        return Some(Rc::clone(node));
    }

    // Horizontal に居る場合
    if let LayoutTree::Horizontal { children, .. } = &(*node.borrow()) {
        // 左に向かって移動している場合
        if direction == Direction::Left {
            return select_to_lower(&children[children.len() - 1], direction);
        }

        // 右に向かって移動している場合，あるいは上下に向かって移動している場合
        return select_to_lower(&children[0], direction);
    }

    // Vertical に居て，かつ上下に移動する場合
    if let LayoutTree::Vertical { children, .. } = &(*node.borrow()) {
        // 上に向かって移動している場合
        if direction == Direction::Up {
            return select_to_lower(&children[children.len() - 1], direction);
        }

        // 下に向かって移動している場合，あるいは左右に向かって移動している場合
        return select_to_lower(&children[0], direction);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::workspace::layout::build::tree::LayoutTree as RawLayoutTree;
    use crate::workspace::layout::control::tree::{LayoutTree, RcLayoutTree};
    use super::{select, Direction};

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

        // 始点取得 (id: 1)
        let tree_ref = tree.borrow();
        let start = if let LayoutTree::Horizontal { children, .. } = &(*tree_ref) {
            let child_ref = children[0].borrow();
            if let LayoutTree::Vertical { children, .. } = &(*child_ref) {
                Rc::clone(&children[0])
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        };

        // 1 --[Down]--> 2
        let dst = move_and_check(&start, Direction::Down, Some(2)).unwrap();

        // 2 --[Right]--> 3
        let dst = move_and_check(&dst, Direction::Right, Some(3)).unwrap();

        // 3 --[Down]--> 4
        let dst = move_and_check(&dst, Direction::Down, Some(4)).unwrap();

        // 4 --[Up]--> 3
        let dst = move_and_check(&dst, Direction::Up, Some(3)).unwrap();

        // 3 --[Left]--> 1
        let dst = move_and_check(&dst, Direction::Left, Some(1)).unwrap();

        // 1 --[Left]--> X
        move_and_check(&dst, Direction::Left, None);

        // 1 --[Up]--> X
        move_and_check(&dst, Direction::Up, None);

    }

    fn move_and_check(tree: &RcLayoutTree, direction: Direction, expected_id: Option<u8>) -> Option<RcLayoutTree> {
        // 移動成功を期待する場合
        if let Some(expected_id) = expected_id {
            let dst = select(tree, direction).unwrap();
            let dst_ref = dst.borrow();
            let dst_id = match &(*dst_ref)  {
                LayoutTree::Widget { id, .. } => id,
                _ => unreachable!(),
            };
            assert_eq!(dst_id, &expected_id);
            return Some(Rc::clone(&dst));
        };

        // 移動失敗を期待する場合
        assert!(select(tree, direction).is_none());
        None
    }
}
