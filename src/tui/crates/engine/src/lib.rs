#![feature(gen_blocks)]

mod layout;
pub mod widget;
pub mod workspace;

use std::cmp::min;
use std::time::Duration;

use crossterm::event::{KeyEvent, KeyCode};
use crossterm::event;
use ratatui::{DefaultTerminal, Frame};

use sb_emu::State as EmuState;

use layout::LayoutManager;
use workspace::Workspace;

pub struct UI {
    // 各 Manager の状態
    layout_man: LayoutManager,
    // widgets_man: WidgetsManager,

    // ワークスペース
    workspace_id: usize,
    workspaces: Vec<Workspace>,

    // エミュレータの状態
    running: bool,
    emu: Option<EmuState>,
    remain_exec_cnt: u32,
}

// Main
impl UI {
    pub fn new<const N: usize>(emu: EmuState, workspaces: [Workspace; N]) -> Self {
        UI {
            layout_man: LayoutManager::default(),
            workspace_id: 0,
            workspaces: workspaces.into_iter().collect::<Vec<_>>(),
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
                let emu = self.workspaces[self.workspace_id].affect(emu);
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
        self.workspaces[self.workspace_id].draw(
            frame,
            &self.layout_man.r#gen(frame),
            self.emu.as_ref().unwrap()
        );
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
            // エミュレータ制御
            KeyCode::Enter => self.remain_exec_cnt = 1,
            KeyCode::Char(' ') => if self.remain_exec_cnt == 0 {
                self.remain_exec_cnt = u32::MAX;
            } else {
                self.remain_exec_cnt = 0;
            },

            // ワークスペース切り替え
            KeyCode::Char(c) if c.is_digit(10) => {
                let num = c.to_digit(10).unwrap();
                let num = if num == 0 { 9 } else { num - 1 };
                self.workspace_id = min(num as usize, self.workspaces.len()-1);
            }

            // 終了
            KeyCode::Char('q') => self.running = false,

            // 各ウィジェットでの処理
            _ => self.workspaces[self.workspace_id].handle_key_event(event),
        }
    }
}
