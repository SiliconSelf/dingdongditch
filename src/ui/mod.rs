//! Contains functionality related to the TUI

use std::{
    io::{self, stdout},
    time::Duration,
};

use actix::prelude::*;
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

/// Actor that handles UI functions
pub(crate) struct UiActor;

impl Actor for UiActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::trace!("UI Actor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::trace!("UI Actor stopped");
    }
}

/// Message to start the UI
#[derive(Message, Debug)]
#[rtype(result = "io::Result<()>")]
pub(crate) struct StartMessage;

impl Handler<StartMessage> for UiActor {
    type Result = io::Result<()>;

    fn handle(
        &mut self,
        msg: StartMessage,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        log::trace!("UI Actor received {msg:?}");
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.draw(ui)?;
        // TODO: Make an actual logic loop
        std::thread::sleep(Duration::from_secs(1));
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}

/// Function to draw the UI
fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!")
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
        frame.size(),
    );
}
