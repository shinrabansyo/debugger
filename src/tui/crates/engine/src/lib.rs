#![feature(gen_blocks)]

pub mod widget;
pub mod workspace;

use std::cmp::min;
use std::time::Duration;

use crossterm::event::{KeyEvent, KeyCode};
use crossterm::event;
use ratatui::widgets::Clear;
use ratatui::text::Text;
use ratatui::{DefaultTerminal, Frame};

use sb_emu::Emulator;

use widget::WidgetView;
use workspace::Workspace;

pub struct UI {
    running: bool,

    // ワークスペース
    workspace_id: usize,
    workspaces: Vec<Workspace>,

    // コマンドライン
    command_mode: bool,
    command: String,
    history: Vec<String>,

    // エミュレータの状態
    emu: Emulator,
    remain_exec_cnt: u32,
}

impl UI {
    pub fn start<const N: usize>(emu: Emulator, workspaces: [Workspace; N]) -> anyhow::Result<()> {
        let mut ui = UI {
            running: true,
            workspace_id: 0,
            workspaces: workspaces.into_iter().collect::<Vec<_>>(),
            command_mode: false,
            command: String::new(),
            history: vec!["Welcome!".to_string()],
            emu,
            remain_exec_cnt: 0,
        };

        let mut terminal = ratatui::init();
        while ui.running {
            ui.emulate()?;
            ui.draw(&mut terminal)?;
            ui.handle_events()?;
        }
        ratatui::restore();

        Ok(())
    }
}

// Emulation
impl UI {
    fn emulate(&mut self) -> anyhow::Result<()> {
        if self.remain_exec_cnt > 0 {
            self.emu.step()?;
            self.workspaces[self.workspace_id].on_emu_updating(&mut self.emu);
            self.remain_exec_cnt -= 1;
        }
        Ok(())
    }
}

// Rendering
impl UI {
    fn draw(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        terminal.draw(|frame| {
            self.draw_workspace(frame);
            if self.command_mode {
                self.draw_command_line(frame);
            }
        })?;
        Ok(())
    }

    fn draw_workspace(&mut self, frame: &mut Frame) {
        self.workspaces[self.workspace_id].draw(frame, &self.emu);
    }

    fn draw_command_line(&mut self, frame: &mut Frame) {
        // 画面中央に 1/4 の大きさで表示
        let mut area = frame.area();
        area.x = area.width / 2 - (area.width / 4);
        area.y = area.height / 2 - (area.height / 4);
        area.width = area.width / 2;
        area.height = area.height / 2;

        // 描画リセット
        frame.render_widget(Clear::default(), area);

        // コマンドライン描画
        let history = self.history.join("\n") + "\n";
        let history = history + format!("> {}", self.command).as_str();
        let command_line = WidgetView::default()
            .title(" Command Line ")
            .body(Text::from(history))
            .selected(true);
        frame.render_widget(command_line, area);
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
        if self.command_mode {
            self.handle_key_event_on_command(event);
        } else {
            self.handle_key_event_on_normal(event);
        }
    }

    fn handle_key_event_on_normal(&mut self, event: KeyEvent) {
        match event.code {
            // エミュレータ制御
            KeyCode::Enter => self.remain_exec_cnt = 1,
            KeyCode::Char(' ') => if self.remain_exec_cnt == 0 {
                self.remain_exec_cnt = u32::MAX;
            } else {
                self.remain_exec_cnt = 0;
            },

            // コマンドモード制御
            KeyCode::Char('c') => self.command_mode = true,

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

    fn handle_key_event_on_command(&mut self, event: KeyEvent) {
        match event.code {
            // コマンドモード離脱
            KeyCode::Esc => self.command_mode = false,

            // コマンド入力
            KeyCode::Char(c) if c.is_ascii() => self.command.push(c),
            KeyCode::Backspace if !self.command.is_empty() => { self.command.pop(); },
            KeyCode::Enter => self.command.clear(),

            _ => {}
        }
    }
}
