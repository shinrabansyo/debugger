#![feature(gen_blocks)]

pub mod widget;
pub mod workspace;

use std::cmp::min;
use std::time::Duration;

use crossterm::event::{KeyEvent, KeyCode};
use crossterm::event;
use ratatui::DefaultTerminal;

use sb_emu::Emulator;

use workspace::Workspace;

pub struct UI {
    // ワークスペース
    workspace_id: usize,
    workspaces: Vec<Workspace>,

    // エミュレータの状態
    running: bool,
    emu: Emulator,
    remain_exec_cnt: u32,
}

impl UI {
    pub fn start<const N: usize>(emu: Emulator, workspaces: [Workspace; N]) -> anyhow::Result<()> {
        let mut ui = UI {
            workspace_id: 0,
            workspaces: workspaces.into_iter().collect::<Vec<_>>(),
            running: true,
            emu,
            remain_exec_cnt: 0,
        };
        ui.run(&mut ratatui::init())?;
        ratatui::restore();
        Ok(())
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while self.running {
            // エミュレータ実行
            if self.remain_exec_cnt > 0 {
                self.emu.step()?;
                self.workspaces[self.workspace_id].on_emu_updating(&mut self.emu);
                self.remain_exec_cnt -= 1;
            }

            // 描画
            terminal.draw(|frame| {
                self.workspaces[self.workspace_id].draw(frame, &self.emu);
            })?;

            // イベント処理
            self.handle_events()?;
        }
        Ok(())
    }

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
            _ => self.workspaces[self.workspace_id].on_key_pressed(event),
        }
    }
}
