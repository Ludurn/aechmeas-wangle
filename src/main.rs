use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

mod db;

use aechmeas_wangle::{
    app::{App, CurrentScreen, GridState},
    ui::ui,
};

fn main() -> std::io::Result<()> {
    let _db = db::connection::open().expect("Failed to open database");

    println!("Database connected");

    ratatui::run(|terminal| {
        let mut app = App::new();
        loop {
            terminal.draw(|frame| ui(frame, &mut app))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break Ok(()),
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break Ok(()),
                    _ => {}
                }

                match app.current_screen {
                    CurrentScreen::Home => {
                        if let Some(screen) = handle_global_keys(key, app.current_screen) {
                            app.set_screen(screen);
                        } else {
                            handle_grid_navigation(key, &mut app.home_grid);
                        }
                    }
                    CurrentScreen::NewSheet => {
                        if let Some(screen) = handle_global_keys(key, app.current_screen) {
                            app.set_screen(screen);
                        } else {
                        }
                    }
                    CurrentScreen::Model => {
                        if let Some(screen) = handle_global_keys(key, app.current_screen) {
                            app.set_screen(screen);
                        } else {
                        }
                    }
                    CurrentScreen::Help => match key.code {
                        KeyCode::Esc => app.go_back(),
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => break Ok(()),
                        KeyCode::Char('n') => app.go_back(),
                        KeyCode::Esc => app.go_back(),
                        _ => {}
                    },
                }
            }
        }
    })
}

fn handle_global_keys(key: KeyEvent, current_screen: CurrentScreen) -> Option<CurrentScreen> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('h'), KeyModifiers::CONTROL) => return Some(CurrentScreen::Help),
        _ => {}
    }

    match key.code {
        KeyCode::Char('L') | KeyCode::Tab => Some(CurrentScreen::next(current_screen)),
        KeyCode::Char('H') => Some(CurrentScreen::previous(current_screen)),
        KeyCode::Char('q') => Some(CurrentScreen::Exiting),
        KeyCode::Char('?') => Some(CurrentScreen::Help),
        _ => None,
    }
}

fn handle_grid_navigation(key: KeyEvent, view: &mut GridState) {
    match key.code {
        KeyCode::Char('k') => view.up(),
        KeyCode::Char('j') => view.down(),
        KeyCode::Char('h') => view.left(),
        KeyCode::Char('l') => view.right(),
        _ => {}
    }
}
