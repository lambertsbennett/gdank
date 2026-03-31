use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen},
    ui::ui,
};


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Overview => match key.code {
                    KeyCode::Char('p') => {
                        app.current_screen = CurrentScreen::Projects;
                    }
                    KeyCode::Char('t') => {
                        app.current_screen = CurrentScreen::Tasks;
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Notes;
                    }
                    KeyCode::Char('q') => {
                        println!("Exiting...");
                        return Ok(true);
                    }
                    _ => {}
                }
                CurrentScreen::Projects => match key.code {
                    KeyCode::Char('t') => {
                        app.current_screen = CurrentScreen::Tasks;
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Notes;
                    }
                    KeyCode::Char('o') => {
                        app.current_screen = CurrentScreen::Overview;
                    }
                    KeyCode::Char('q') => {
                        println!("Exiting...");
                        return Ok(true);
                    }

                    _ => {}
                }
                CurrentScreen::Tasks  => match key.code {
                        KeyCode::Char('p') => {
                            app.current_screen = CurrentScreen::Projects;
                        }
                        KeyCode::Char('n') => {
                            app.current_screen = CurrentScreen::Notes;
                        }
                        KeyCode::Char('o') => {
                            app.current_screen = CurrentScreen::Overview;
                        }
                        KeyCode::Char('q') => {
                            println!("Exiting...");
                            return Ok(true);
                        }
                        _ => {}
                    }
                CurrentScreen::Notes  => match key.code {
                        KeyCode::Char('p') => {
                            app.current_screen = CurrentScreen::Projects;
                        }
                        KeyCode::Char('t') => {
                            app.current_screen = CurrentScreen::Tasks;
                        }
                        KeyCode::Char('o') => {
                            app.current_screen = CurrentScreen::Overview;
                        }
                        KeyCode::Char('q') => {
                            println!("Exiting...");
                            return Ok(true);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

                    