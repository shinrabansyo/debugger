use ratatui::layout::{Constraint, Direction, Rect};
use ratatui::layout::Layout as RataLayout;
use ratatui::Frame;

pub struct Layout {
    pub inst: Rect,
    pub output: Rect,
    pub state: Rect,
    pub memory: Rect,
    pub help: Rect,
}

#[derive(Default)]
pub struct LayoutManager;

impl LayoutManager {
    pub fn gen(&self, frame: &Frame) -> Layout {
        // +------------------------------+
        // |              |               |
        // |              |     State     |
        // | Instructions |               |
        // |              |---------------|
        // |--------------|     Memory    |
        // |    Output    |               |
        // |--------------+---------------|
        // |         Help Message         |
        // +------------------------------+

        let outer_layout = RataLayout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(100),
                Constraint::Length(1),
            ])
            .split(frame.area());

        let upper_layout = RataLayout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(outer_layout[0]);

        let left_layout = RataLayout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(upper_layout[0]);

        let right_layout = RataLayout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(upper_layout[1]);

        Layout {
            inst: left_layout[0],
            output: left_layout[1],
            state: right_layout[0],
            memory: right_layout[1],
            help: outer_layout[1],
        }
    }
}
