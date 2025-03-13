#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LayoutTree {
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
    pub(crate) fn size(&self) -> u16 {
        match self {
            LayoutTree::Horizontal { size, .. } => *size,
            LayoutTree::Vertical { size, .. } => *size,
            LayoutTree::Widget { size, .. } => *size,
        }
    }
}
