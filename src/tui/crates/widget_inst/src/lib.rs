use std::cmp::max;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Text, Line, Span};

use sb_disasm::disassemble;
use sb_emu::State as EmuState;

use sb_dbg_tui_engine::widget::{Widget, WidgetState};

#[derive(Default)]
pub struct InstState {
    offset: i32,
}

impl WidgetState for InstState {
    fn draw(&self, area: &Rect, emu: &EmuState) -> Widget {
        let max_lines = area.height as i32;

        let mut lines = vec![];
        for row in 0..max_lines {
            let mut line = vec![];

            // 表示対象命令のアドレスを計算
            let pc = emu.pc as i32;
            let addr = self.offset + pc + row * 6;
            if addr < 0 {
                lines.push(Line::from(line));
                continue;
            }
            let addr = addr as usize;

            // 命令アドレス
            line.push(Span::styled(
                format!("0x{:08x}: ", addr),
                Style::new().fg(Color::Yellow),
            ));

            // 出力幅調整用スペースの準備
            let padding_size = max(0, area.width as i32 - 12 - 32 - 16) as usize;
            let padding = " ".repeat(padding_size);

            // 命令
            let raw_inst = emu.imem.read::<6>(addr).unwrap();
            let assembly = disassemble(raw_inst);
            if addr == emu.pc as usize {
                line.push(Span::styled(
                    format!("{:32}{}0x{:012x}", assembly, padding, raw_inst),
                    Style::new().fg(Color::Red).underlined().bold(),
                ));
            } else {
                line.push(Span::styled(
                    format!("{:32}{}0x{:012x}", assembly, padding, raw_inst),
                    Style::new().fg(Color::White),
                ));
            }

            lines.push(Line::from(line));
        }

        Widget::default().title(" Instructions ").body(Text::from(lines))
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        self.offset = match event.code {
            KeyCode::Up => self.offset - 6,
            KeyCode::Down => self.offset + 6,
            _ => self.offset,
        };
    }
}
