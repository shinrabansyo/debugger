mod widget;
mod layout;

use crossterm::event::{KeyEvent, KeyCode};
use crossterm::event;
use ratatui::{DefaultTerminal, Frame};

use widget::{InstView, OutputView, StateView, MemView};
use layout::LayoutManager;

pub struct UI {
    layout_man: LayoutManager,
    running: bool,
}

// Main
impl UI {
    pub fn new() -> Self {
        UI {
            layout_man: LayoutManager::default(),
            running: true,
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
        let layout = self.layout_man.gen(frame);

        let inst_view = InstView {};
        frame.render_widget(inst_view, layout.inst);

        let output_view = OutputView {};
        frame.render_widget(output_view, layout.output);

        let state_view = StateView {};
        frame.render_widget(state_view, layout.state);

        let mem_view = MemView {};
        frame.render_widget(mem_view, layout.memory);
    }
}

// Event Handling
impl UI {
    fn handle_events(&mut self) -> anyhow::Result<()> {
        match event::read()? {
            event::Event::Key(event) => self.handle_key_event(event)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> anyhow::Result<()> {
        match event.code {
            KeyCode::Char('q') => self.running = false,
            _ => {}
        }
        Ok(())
    }
}
