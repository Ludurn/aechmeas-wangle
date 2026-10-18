use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
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
        let paragraph_color = match (&app.current_screen, label) {
            (CurrentScreen::Home, "HOME") => Color::Magenta,
            _ => Color::White,
        };
        frame.render_widget(
            Paragraph::new(label)
                .block(Block::bordered())
                .centered()
                .style(paragraph_color),
            nav_layout[index],
        );
    });

    if let CurrentScreen::Home = app.current_screen {
        frame.render_widget("Home Screen", default_layout[1]);
    }

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
