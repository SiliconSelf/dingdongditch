#![doc = include_str!("../README.md")]

use std::{io::{self, Stdout}, thread, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use ratatui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders}};

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Ding Dong Ditch")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    thread::sleep(Duration::from_secs(1));
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
