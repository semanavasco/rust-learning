mod state;
mod ui;

use std::error::Error;
use std::io;

use ratatui::Terminal;
use ratatui::crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::{Backend, CrosstermBackend};

use crate::state::{CurrentScreen, CurrentlyEditing, State};
use crate::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // Setting up terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Creating and running the app with state
    let mut state = State::new();
    let res = run_app(&mut terminal, &mut state);

    // Restoring terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Ok(do_print) => {
            if do_print {
                state.print_json()?;
            }
        }
        Err(err) => {
            println!("{err:?}");
        }
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip other events
                continue;
            }

            match state.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        state.current_screen = CurrentScreen::Editing;
                        state.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        state.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &state.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    state.currently_editing = Some(CurrentlyEditing::Value);
                                }
                                CurrentlyEditing::Value => {
                                    state.save_key_value();
                                    state.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &state.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    state.key_input.pop();
                                }
                                CurrentlyEditing::Value => {
                                    state.value_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        state.current_screen = CurrentScreen::Main;
                        state.currently_editing = None;
                    }
                    KeyCode::Tab => {
                        state.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &state.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => state.key_input.push(value),
                                CurrentlyEditing::Value => state.value_input.push(value),
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
