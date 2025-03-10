use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};

use sb_emu::State as EmuState;

use crate::ui::widget::{Widget, WidgetState};

#[derive(Default)]
pub struct RegisterState {
    selected: bool,
}

impl WidgetState for RegisterState {
    fn draw(&self, _: &Rect, emu: &EmuState) -> Widget {
        let mut lines = vec![];

        // PC 表示
        let pc_line = Line::from(vec![
            Span::styled("PC: ", Style::new().fg(Color::Yellow)),
            Span::styled(format!("0x{:08x}", emu.pc), Style::new().fg(Color::White)),
        ]);
        lines.push(pc_line);

        // レジスタ表示
        for row in 0..8 {
            let mut reg_items = vec![];
            for reg in row*4..row*4+4 {
                reg_items.push(Span::styled(
                    format!("r{:02}: ", reg),
                    Style::new().fg(Color::Yellow),
                ));
                reg_items.push(Span::styled(
                    format!("0x{:08x} ", emu.regs.read(reg).unwrap()),
                    Style::new().fg(Color::White),
                ));
            }
            lines.push(Line::from(reg_items));
        }

        Widget::default()
            .selected(self.selected)
            .title(" Register ")
            .body(Text::from(lines))
    }

    fn handle_key_event(&mut self, _: KeyEvent) {
        // do nothing
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
