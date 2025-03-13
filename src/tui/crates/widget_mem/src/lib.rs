use std::cmp::max;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text, Span};

use sb_dbg::Debugger;

use sb_dbg_tui_engine::widget::{Widget, WidgetView};

#[derive(Default)]
pub struct Mem {
    offset: i32,
}

impl Widget for Mem {
    fn draw(&self, area: &Rect, debugger: &Debugger) -> WidgetView {
        let max_lines = area.height as i32;

        let mut lines = vec![];
        for row in 0..max_lines {
            let mut line = vec![];

            // 行アドレス
            let row_addr = (self.offset + row * 16) as usize;
            line.push(Span::styled(
                format!("0x{:08x}:", row_addr),
                Style::new().fg(Color::Yellow)
            ));

            // バイト列
            for col in 0..16 {
                let addr = row_addr + col;
                let byte = debugger.dmem.read_byte(addr).unwrap();
                line.push(Span::styled(
                    format!(" {:02x}", byte),
                    Style::new().fg(Color::White)
                ));
            }

            // 出力幅調整
            let padding_size = max(0, area.width as i32 - 10 - 47 - 20) as usize;
            let padding = " ".repeat(padding_size);
            line.push(Span::from(padding));

            // ASCII 表示
            for col in 0..16 {
                let addr = row_addr + col;
                let byte = debugger.dmem.read_byte(addr).unwrap();
                let c = if byte.is_ascii_alphanumeric() {
                    byte as char
                } else {
                    '.'
                };
                line.push(Span::styled(
                    format!("{}", c),
                    Style::new().fg(Color::White)
                ));
            }

            lines.push(Line::from(line));
        }

        WidgetView::default().title(" Memory ").body(Text::from(lines))
    }

    fn on_key_pressed(&mut self, event: KeyEvent) {
        self.offset = match event.code {
            KeyCode::Up => max(0, self.offset - 16),
            KeyCode::Down => self.offset + 16,
            _ => self.offset,
        };
    }
}
