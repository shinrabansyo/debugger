use crossterm::event::{KeyEvent, KeyCode};
use crossterm::event;
use ratatui::widgets::Paragraph;
use ratatui::style::Stylize;
use ratatui::{DefaultTerminal, Frame};

pub struct UI {
    running: bool,
}

// Main
impl UI {
    pub fn new() -> Self {
        UI {
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
        let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .white()
            .on_blue();
        frame.render_widget(greeting, frame.area());
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
