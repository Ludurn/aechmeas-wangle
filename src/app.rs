#[derive(Clone, Copy)]
pub enum CurrentScreen {
    Home,
    NewSheet,
    Model,
    Help,
    Exiting,
}

pub struct GridState {
    pub visible_rows: usize,
    pub visible_cols: usize,
    pub selected: usize,
    pub scroll_row: usize,
    pub total_items: usize,
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

impl GridState {
    pub fn new() -> GridState {
        GridState {
            visible_rows: 0,
            visible_cols: 0,
            selected: 0,
            scroll_row: 0,
            total_items: 0,
        }
    }

    pub fn update_layout(&mut self, visible_rows: usize, visible_cols: usize, total_items: usize) {
        self.visible_rows = visible_rows;
        self.visible_cols = visible_cols;
        self.total_items = total_items;

        if total_items == 0 {
            self.selected = 0;
            self.scroll_row = 0;
        } else {
            self.selected = self.selected.min(total_items - 1);
            self.ensure_selected_is_visible();
        }
    }

    fn ensure_selected_is_visible(&mut self) {
        if self.visible_cols == 0 || self.visible_rows == 0 {
            return;
        }

        let selected_row = self.selected / self.visible_cols;
        let last_visible_row = self.scroll_row + self.visible_rows.saturating_sub(1);

        if selected_row < self.scroll_row {
            self.scroll_row = selected_row;
        } else if selected_row > last_visible_row {
            self.scroll_row = selected_row - self.visible_rows + 1;
        }
    }

    pub fn up(&mut self) {
        if self.visible_cols > 0 && self.selected >= self.visible_cols {
            self.selected -= self.visible_cols;
            self.ensure_selected_is_visible();
        }
    }

    pub fn down(&mut self) {
        if self.total_items == 0 || self.visible_cols == 0 {
            return;
        }

        let next = self.selected + self.visible_cols;
        let last = self.total_items - 1;

        if next <= last {
            self.selected = next;
        } else if self.selected % self.visible_cols > last % self.visible_cols {
            self.selected = last;
        }

        self.ensure_selected_is_visible();
    }

    pub fn left(&mut self) {
        if self.visible_cols > 0 && self.selected % self.visible_cols > 0 {
            self.selected -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.visible_cols == 0 {
            return;
        }

        let next = self.selected + 1;

        if next % self.visible_cols != 0 && next < self.total_items {
            self.selected = next;
        }
    }
}

impl Default for GridState {
    fn default() -> Self {
        Self::new()
    }
}
