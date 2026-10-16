use crossterm::event::{self, Event, KeyCode};

mod app;
mod ui;
use crate::app::{App, CurrentScreen};
use crate::ui::ui;

fn main() -> std::io::Result<()> {
    ratatui::run(|mut terminal| {
        let mut app = App::new();
        loop {
            terminal.draw(|frame| ui(frame, &mut app))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match app.current_screen {
                    CurrentScreen::Home => match key.code {
                        KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => break Ok(()),
                        KeyCode::Char('n') => app.current_screen = CurrentScreen::Home,
                        _ => {}
                    },
                }
            }
        }
    })
}
