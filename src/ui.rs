mod widget;
mod layout;

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
}

// Main
impl UI {
    pub fn new(emu: EmuState) -> Self {
        UI {
            layout_man: LayoutManager::default(),
            widgets_man: WidgetsManager::new(),
            running: true,
            emu: Some(emu),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
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

        frame.render_widget(widegts.inst_view, layout.inst);
        frame.render_widget(widegts.output_view, layout.output);
        frame.render_widget(widegts.state_view, layout.state);
        frame.render_widget(widegts.mem_view, layout.memory);
    }
}

// Event Handling
impl UI {
    fn handle_events(&mut self) -> anyhow::Result<()> {
        match event::read()? {
            event::Event::Key(event) => self.handle_key_event(event),
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Enter => {
                let emu = self.emu.take().unwrap();
                let emu = sb_emu::step(emu).unwrap();
                self.emu = Some(emu);
            }
            KeyCode::Char('q') => self.running = false,
            _ => self.widgets_man.handle_key_event(event),
        }
    }
}
