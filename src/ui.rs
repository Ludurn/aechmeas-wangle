use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, Wrap},
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    let default_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let nav_layout = Layout::default()
        .direction(Direction::Horizontal)
        .flex(Flex::SpaceEvenly)
        .constraints([
            Constraint::Max(12),
            Constraint::Max(13),
            Constraint::Max(13),
        ])
        .split(default_layout[0]);

    let nav_labels = ["HOME", "NEW SHEET", "MODEL"];

    nav_labels.iter().enumerate().for_each(|(index, &label)| {
        let p_color = match (&app.current_screen, label) {
            (CurrentScreen::Home, "HOME") => Color::Magenta,
            (CurrentScreen::NewSheet, "NEW SHEET") => Color::Magenta,
            (CurrentScreen::Model, "MODEL") => Color::Magenta,
            _ => Color::White,
        };
        frame.render_widget(
            Paragraph::new(label)
                .block(Block::bordered())
                .centered()
                .style(p_color),
            nav_layout[index],
        );
    });

    let main_block = Block::bordered();
    let main_chunk = main_block.inner(default_layout[1]);
    frame.render_widget(main_block, default_layout[1]);

    let (w, h) = (frame.area().width, frame.area().height);

    let cols = match w {
        201.. => 5,
        151..=200 => 4,
        131..=150 => 3,
        81..=130 => 2,
        31..=80 => 1,
        _ => 1,
    };

    let rows = match h {
        51.. => 5,
        41..=50 => 4,
        31..=40 => 3,
        24..=30 => 2,
        11..=23 => 1,
        _ => 1,
    };

    let grid: Vec<Vec<Rect>> = grid(cols, rows, main_chunk);

    if let CurrentScreen::Home = app.current_screen {
    }

    if let CurrentScreen::Help = app.current_screen {
        let popup_block = Block::default()
            .title(Line::from("Commands").style(Style::default().fg(Color::Magenta)))
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Reset));

        let area = centered_rect(50, 15, frame.area());
        let inner = popup_block.inner(area);

        let mut help_entries = vec![
            ["Command", "Shortcut", "Description"],
            ["Quit", "[\"C-q\", \"C-c\"]", "quit immediately"],
            ["Quit Menu", "[\"q\"]", "open the quit confirmation popup"],
            ["Close Popup", "[\"esc\"]", "close a popup"],
            ["Previous Tab", "[\"H\"]", "go to previous tab"],
            ["Next Tab", "[\"L\", \"tab\"]", "go to next tab"],
            ["Help", "[\"?\", \"C-h\"]", "open the help popup"],
        ];

        let contextual_help_entries: Option<Vec<[&str; 3]>> = match app.previous_screen {
            Some(CurrentScreen::Home) => Some(vec![["", "", ""]]),
            _ => None,
        };

        if let Some(vec_text) = contextual_help_entries {
            help_entries.extend(vec_text);
        }

        let rows = help_entries
            .iter()
            .skip(1)
            .map(|row| Row::new(vec![row[0], row[1], row[2]]));

        let table = Table::new(
            rows,
            [
                Constraint::Length(30),
                Constraint::Length(20),
                Constraint::Fill(1),
            ],
        )
        .header(Row::new(help_entries[0]));

        frame.render_widget(popup_block, area);
        frame.render_widget(table, inner);
    }

    let key_hint_layout = Layout::default()
        .direction(Direction::Horizontal)
        .flex(Flex::Center)
        .constraints([Constraint::Max(70)])
        .split(default_layout[2]);

    let global_key_hints = "[H] Previous tab | [L] or [TAB] Next tab | [q] Quit | [?] Help";

    frame.render_widget(
        Paragraph::new(global_key_hints)
            .block(Block::bordered())
            .centered()
            .style(Color::Yellow),
        key_hint_layout[0],
    );

    if let CurrentScreen::Exiting = app.current_screen {
        clear_frame(frame);

        let popup_block = Block::default()
            .title("Quit?")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Reset));

        let area = centered_rect(50, 8, frame.area());
        let line_width = area.width.saturating_sub(2);

        let exit_text = Text::from(vec![
            Line::from("Would you like to quit the app?"),
            Line::from("─".repeat(line_width as usize)),
            Line::from(""),
            Line::from("[Y] Yes    [N] No"),
        ]);

        let exit_paragraph = Paragraph::new(exit_text)
            .style(Style::default().fg(Color::Blue))
            .alignment(Alignment::Center)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        frame.render_widget(exit_paragraph, area);
    }
}

fn clear_frame(frame: &mut Frame) {
    frame.render_widget(Clear, frame.area());
}

fn centered_rect(x: u16, y: u16, r: Rect) -> Rect {
    r.centered(Constraint::Percentage(x), Constraint::Length(y))
}

fn grid(cols: usize, rows: usize, area: Rect) -> Vec<Vec<Rect>> {
    let columns = Layout::horizontal(vec![Constraint::Fill(1); cols]).split(area);

    columns
        .iter()
        .map(|col| {
            Layout::vertical(vec![Constraint::Fill(1); rows])
                .split(*col)
                .to_vec()
        })
        .collect()
}
