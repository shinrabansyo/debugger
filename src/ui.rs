mod widget;
mod layout;

use std::time::Duration;

use crossterm::event::{KeyEvent, KeyCode};
use crossterm::event;
use ratatui::{DefaultTerminal, Frame};

use sb_emu::State as EmuState;

use widget::WidgetsManager;
use layout::LayoutManager;

pub struct UI {
    // 各 Manager の状態
    layout_man: LayoutManager,
    widgets_man: WidgetsManager,

    // 全体の状態
    running: bool,
    emu: Option<EmuState>,
    remain_exec_cnt: u32,
}

// Main
impl UI {
    pub fn new(emu: EmuState) -> Self {
        UI {
            layout_man: LayoutManager::default(),
            widgets_man: WidgetsManager::new(),
            running: true,
            emu: Some(emu),
            remain_exec_cnt: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while self.running {
            // エミュレータ実行
            if self.remain_exec_cnt > 0 {
                let emu = self.emu.take().unwrap();
                let emu = sb_emu::step(emu).unwrap();
                self.emu = Some(emu);
                self.remain_exec_cnt -= 1;
            }

            // 描画
            terminal.draw(|frame| self.draw(frame))?;

            // イベント処理
            self.handle_events()?;
        }
        Ok(())
    }
}

// Rendering
impl UI {
    fn draw(&mut self, frame: &mut Frame) {
        let widegts = self.widgets_man.gen_widgets(self.emu.as_ref().unwrap());
        let layout = self.layout_man.gen(frame);

        frame.render_widget(widegts.inst, layout.inst);
        frame.render_widget(widegts.device, layout.device);
        frame.render_widget(widegts.state, layout.state);
        frame.render_widget(widegts.mem, layout.memory);
        frame.render_widget(widegts.help, layout.help);
    }
}

// Event Handling
impl UI {
    fn handle_events(&mut self) -> anyhow::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                event::Event::Key(event) => self.handle_key_event(event),
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Enter => self.remain_exec_cnt = 1,
            KeyCode::Char(' ') => if self.remain_exec_cnt == 0 {
                self.remain_exec_cnt = u32::MAX;
            } else {
                self.remain_exec_cnt = 0;
            },
            KeyCode::Char('q') => self.running = false,
            _ => self.widgets_man.handle_key_event(event),
        }
    }
}
