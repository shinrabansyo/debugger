mod widget;
mod layout;

use crossterm::event::{KeyEvent, KeyCode};
use crossterm::event;
use ratatui::{DefaultTerminal, Frame};

use widget::WidgetsManager;
use layout::LayoutManager;

pub struct UI {
    layout_man: LayoutManager,
    widgets_man: WidgetsManager,
    running: bool,
}

// Main
impl UI {
    pub fn new() -> Self {
        UI {
            layout_man: LayoutManager::default(),
            widgets_man: WidgetsManager::new(),
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
        let widegts = self.widgets_man.gen_widgets();
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
        self.widgets_man.handle_key_event(event);
        match event.code {
            KeyCode::Char('q') => self.running = false,
            _ => {}
        }
    }
}
