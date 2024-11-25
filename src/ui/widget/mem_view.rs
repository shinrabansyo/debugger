use std::cmp::max;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Text, Span};

use sb_emu::State as EmuState;

pub struct MemView {
    selected: bool,
    text: Text<'static>,
}

impl Widget for MemView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
                .title(Line::from(" Memory ".bold()).centered())
                .border_set(if self.selected { border::THICK } else { border::ROUNDED });

        Paragraph::new(self.text)
            .block(block)
            .render(area, buf);
    }
}

pub struct MemViewState {
    selected: bool,
    offset: i32,
}

impl MemViewState {
    pub fn new(selected: bool) -> Self {
        MemViewState {
            selected,
            offset: 0,
        }
    }

    pub fn gen_widget(&self, emu: &EmuState) -> MemView {
        let mut lines = vec![];
        for row in 0..20 {
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
                let byte = emu.dmem.read_byte(addr).unwrap();
                line.push(Span::styled(
                    format!(" {:02x}", byte),
                    Style::new().fg(Color::White)
                ));
            }

            lines.push(Line::from(line));
        }

        MemView {
            selected: self.selected,
            text: Text::from(lines),
        }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        self.offset = match event.code {
            KeyCode::Up => max(0, self.offset - 16),
            KeyCode::Down => self.offset + 16,
            _ => self.offset,
        };
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}
