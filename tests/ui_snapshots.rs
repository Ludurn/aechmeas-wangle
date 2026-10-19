#[cfg(test)]
mod tests {

    use aechmeas_wangle::{
        app::{App, CurrentScreen},
        ui::ui,
    };

    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};

    const TERMINAL_WIDTH: u16 = 80;
    const TERMINAL_HEIGHT: u16 = 24;

    fn assert_screen_snapshot(name: &str, app: &App) {
        let backend = TestBackend::new(TERMINAL_WIDTH, TERMINAL_HEIGHT);

        let mut terminal = Terminal::new(backend).expect("failed to create test terminal");

        terminal
            .draw(|frame| ui(frame, app))
            .expect("failed to render UI");

        assert_snapshot!(name, terminal.backend());
    }

    #[test]
    fn home_screen() {
        let app = App {
            current_screen: CurrentScreen::Home,
            ..App::default()
        };

        assert_screen_snapshot("home_screen", &app);
    }

    #[test]
    fn help_screen() {
        let app = App {
            current_screen: CurrentScreen::Help,
            ..App::default()
        };

        assert_screen_snapshot("help_screen", &app);
    }

    #[test]
    fn exiting_screen() {
        let app = App {
            current_screen: CurrentScreen::Exiting,
            ..App::default()
        };

        assert_screen_snapshot("exiting_screen", &app);
    }
}
