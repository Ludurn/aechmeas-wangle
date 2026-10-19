#[derive(Clone, Copy)]
pub enum CurrentScreen {
    Home,
    NewSheet,
    Model,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub previous_screen: Option<CurrentScreen>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Home,
            previous_screen: None,
        }
    }

    pub fn set_screen(&mut self, next: CurrentScreen) {
        self.previous_screen = Some(self.current_screen);
        self.current_screen = next;
    }

    pub fn go_back(&mut self) {
        if let Some(screen) = self.previous_screen.take() {
            self.current_screen = screen;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl CurrentScreen {
    pub fn next(self) -> Self {
        match self {
            CurrentScreen::Home => CurrentScreen::NewSheet,
            CurrentScreen::NewSheet => CurrentScreen::Model,
            CurrentScreen::Model => CurrentScreen::Home,
            _ => CurrentScreen::Exiting,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            CurrentScreen::Home => CurrentScreen::Model,
            CurrentScreen::NewSheet => CurrentScreen::Home,
            CurrentScreen::Model => CurrentScreen::NewSheet,
            _ => CurrentScreen::Exiting,
        }
    }
}
