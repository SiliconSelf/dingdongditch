//! Contains functionality related to the TUI

use actix::prelude::*;

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
#[rtype(result = "()")]
pub(crate) struct StartMessage;

impl Handler<StartMessage> for UiActor {
    type Result = ();
    fn handle(&mut self, msg: StartMessage, _ctx: &mut Context<Self>) -> Self::Result {
        log::trace!("UI Actor received {msg:?}");
    }
}