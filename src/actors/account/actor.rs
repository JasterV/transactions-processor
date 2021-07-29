use crate::{models::{account::Account, actor::Actor, transaction::Transaction}};
use std::{collections::HashMap};
use crate::actors::account::messages::Command;

pub struct AccountActor {
    account: Account,
    transactions: HashMap<u32, Transaction>,
}

impl AccountActor {
    pub fn new(client_id: u16) -> Self {
        Self {
            account: Account::new(client_id),
            transactions: HashMap::new(),
        }
    }
}

impl Actor<Command> for AccountActor {
    fn handle(&mut self, command: Command) {
        println!("command: {:#?}", command);
        if let Command::Stop(rx) = command {
            let _ = rx.send(self.account.clone());
        }
    }
}