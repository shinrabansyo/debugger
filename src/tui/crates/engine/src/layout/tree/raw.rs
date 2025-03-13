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
