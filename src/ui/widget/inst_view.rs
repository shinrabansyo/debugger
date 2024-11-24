use std::cmp::max;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Text, Line, Span};

use sb_emu::State as EmuState;

pub struct InstView {
    selected: bool,
    text: Text<'static>,
}

impl Widget for InstView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
                .title(Line::from(" Instructions ".bold()).centered())
                .border_set(if self.selected { border::THICK } else { border::ROUNDED });

        Paragraph::new(self.text)
            .block(block)
            .render(area, buf);
    }
}

pub struct InstViewState {
    selected: bool,
    offset: i32,
}

impl InstViewState {
    pub fn new(selected: bool) -> Self {
        InstViewState {
            selected,
            offset: 0,
        }
    }

    pub fn gen_widget(&self, emu: &EmuState) -> InstView {
        let mut lines = vec![];
        for row in 0..24 {
            let mut line = vec![];

            // 命令アドレス
            let addr = (self.offset + row * 6) as usize;
            line.push(Span::styled(
                format!("0x{:08x}: ", addr),
                Style::new().fg(Color::Yellow),
            ));

            // 命令
            let raw_inst = emu.imem.read::<6>(addr).unwrap();
            let assembly = "(TODO)";
            if addr == emu.pc as usize {
                line.push(Span::styled(
                    format!("{:32} 0x{:08b}", assembly, raw_inst),
                    Style::new().fg(Color::Red).underlined().bold(),
                ));
            } else {
                line.push(Span::styled(
                    format!("{:32} 0x{:08b}", assembly, raw_inst),
                    Style::new().fg(Color::White),
                ));
            }

            lines.push(Line::from(line));
        }

        InstView {
            selected: self.selected,
            text: Text::from(lines),
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        self.offset = match event.code {
            KeyCode::Up => max(0, self.offset - 6),
            KeyCode::Down => self.offset + 6,
            _ => self.offset,
        };
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
