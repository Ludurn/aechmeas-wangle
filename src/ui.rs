use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    frame.render_widget("Hello World!", frame.area());

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
