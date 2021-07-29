use std::collections::HashMap;
use actix::{Actor, ActorContext, Context, Handler, MessageResult, dev::MessageResponse};

use crate::{messages::{Command, Stop}, models::{account::Account, transaction::Transaction}};

pub struct AccountActor {
    account: Account,
    transactions: HashMap<u32, Transaction>
}

impl AccountActor {
    pub fn new(client_id: u16) -> Self {
        Self {
            account: Account::new(client_id),
            transactions: HashMap::new()
        }
    }
}

impl Actor for AccountActor {
    type Context = Context<Self>;

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        actix::Running::Stop
    }
}

// now we need to implement `Handler` on `Calculator` for the `Sum` message.
impl Handler<Command> for AccountActor {
    type Result = (); // <- Message response type

    fn handle(&mut self, msg: Command, ctx: &mut Context<Self>) -> Self::Result {
        // TODO: Implement commands
    }
}

impl Handler<Stop> for AccountActor {
    type Result = MessageResult<Stop>; // <- Message response type

    fn handle(&mut self, _msg: Stop, ctx: &mut Context<Self>) -> Self::Result {
        ctx.stop();
        MessageResult(self.account.clone())        
    }
}
