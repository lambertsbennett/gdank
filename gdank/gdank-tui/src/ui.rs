use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders,List, ListItem, Paragraph},
    Frame,
};    

use crate::app::{App, CurrentScreen};



fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
    ])
    .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Overview",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();
    match app.current_screen {
            CurrentScreen::Overview => {
                    for project in app.projects.iter() {
                        list_items.push(ListItem::new(Line::from(Span::styled(
                            format!("{: <25} : {}", project.name, project.description),
                            Style::default().fg(Color::Yellow),
                        ))));
                    }
            }
            CurrentScreen::Projects => {
                    for project in app.projects.iter() {
                        list_items.push(ListItem::new(Line::from(Span::styled(
                            format!("{: <25} : {}", project.name, project.description),
                            Style::default().fg(Color::Yellow),
                        ))));
                    }
            }
            CurrentScreen::Tasks => {
                    for project in app.projects.iter() {
                        list_items.push(ListItem::new(Line::from(Span::styled(
                            format!("{: <25} : {}", project.name, project.description),
                            Style::default().fg(Color::Yellow),
                        ))));
                    }
            }
            CurrentScreen::Notes => {
                    for project in app.projects.iter() {
                        list_items.push(ListItem::new(Line::from(Span::styled(
                            format!("{: <25} : {}", project.name, project.description),
                            Style::default().fg(Color::Yellow),
                        ))));
                    }
        }
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Overview => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Projects => Span::styled("Projects Mode", Style::default().fg(Color::Yellow)),
            CurrentScreen::Tasks => Span::styled("Tasks Mode", Style::default().fg(Color::LightBlue)),
            CurrentScreen::Notes => Span::styled("Notes Mode", Style::default().fg(Color::LightMagenta)),
        }
        .to_owned()
        

    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

        let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Overview => Span::styled(
                "(q) to quit / (p) projects / (t) tasks",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Projects => Span::styled(
                "(q) to quit / (t) tasks / (o) overview",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Tasks => Span::styled(
                "(q) to quit / (p) projects / (o) overview",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Notes => Span::styled(
                "(q) to quit / (p) projects / (o) overview",
                Style::default().fg(Color::Red),
            ),
        }
    };
    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}
