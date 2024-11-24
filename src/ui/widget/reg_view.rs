use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block, Paragraph};
use ratatui::symbols::border;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};

use sb_emu::State as EmuState;

pub struct RegisterView {
    selected: bool,
    text: Text<'static>,
}

impl Widget for RegisterView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
                .title(Line::from(" Register ".bold()).centered())
                .border_set(if self.selected { border::THICK } else { border::ROUNDED });

        Paragraph::new(self.text)
            .block(block)
            .render(area, buf);
    }
}

pub struct RegisterViewState {
    selected: bool,
}

impl RegisterViewState {
    pub fn new(selected: bool) -> Self {
        RegisterViewState {
            selected,
        }
    }

    pub fn gen_widget(&self, emu: &EmuState) -> RegisterView {
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

        RegisterView {
            selected: self.selected,
            text: Text::from(lines),
        }
    }

    pub fn handle_key_event(&mut self, _: KeyEvent) {
        // do nothing
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}