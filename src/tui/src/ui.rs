mod layout;
mod widget;
mod workspace;

use std::time::Duration;

use crossterm::event::{KeyEvent, KeyCode};
use crossterm::event;
use ratatui::{DefaultTerminal, Frame};

use sb_emu::State as EmuState;

use layout::LayoutManager;
use workspace::{Workspace, WorkspaceBuilder};

pub struct UI {
    // 各 Manager の状態
    layout_man: LayoutManager,
    // widgets_man: WidgetsManager,
    workspace: Workspace,

    // 全体の状態
    running: bool,
    emu: Option<EmuState>,
    remain_exec_cnt: u32,
}

// Main
impl UI {
    pub fn new(emu: EmuState) -> Self {
        let workspace = WorkspaceBuilder::default()
            .widget((0, 0), Box::new(widget::inst::InstState::default()))
            .widget((0, 1), Box::new(widget::device::DeviceState::default()))
            .widget((1, 0), Box::new(widget::reg::RegisterState::default()))
            .widget((1, 1), Box::new(widget::mem::MemState::default()))
            .build();

        UI {
            layout_man: LayoutManager::default(),
            workspace,
            running: true,
            emu: Some(emu),
            remain_exec_cnt: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while self.running {
            // エミュレータ実行
            if self.remain_exec_cnt > 0 {
                // 1ステップ実行
                let emu = self.emu.take().unwrap();
                let emu = self.workspace.affect(emu);
                let emu = sb_emu::step(emu).unwrap(); // (命令実行)

                // 状態更新
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
        let layout = self.layout_man.r#gen(frame);
        let widgets = self.workspace.draw(&layout, self.emu.as_ref().unwrap());

        let mut a = widgets.into_iter();

        frame.render_widget(a.next().unwrap(), layout.inst);
        frame.render_widget(a.next().unwrap(), layout.device);
        frame.render_widget(a.next().unwrap(), layout.state);
        frame.render_widget(a.next().unwrap(), layout.memory);
        frame.render_widget(a.next().unwrap(), layout.mode);
        frame.render_widget(a.next().unwrap(), layout.help);
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
            _ => self.workspace.handle_key_event(event),
        }
    }
}
