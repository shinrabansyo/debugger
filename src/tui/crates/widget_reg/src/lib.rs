use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};

use sb_dbg::Debugger;

use sb_dbg_tui_engine::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Register;

impl Widget for Register {
    fn draw(&self, _: &Rect, debugger: &Debugger) -> WidgetView {
        let mut lines = vec![];

        // PC 表示
        let pc_line = Line::from(vec![
            Span::styled("PC: ", Style::new().fg(Color::Yellow)),
            Span::styled(format!("0x{:08x}", debugger.pc), Style::new().fg(Color::White)),
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
                    format!("0x{:08x} ", debugger.regs.read(reg).unwrap()),
                    Style::new().fg(Color::White),
                ));
            }
            lines.push(Line::from(reg_items));
        }

        WidgetView::default().title(" Register ").body(Text::from(lines))
    }
}
