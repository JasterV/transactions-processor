use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use crate::actors::account::messages::Command as AccountCommand;
use super::messages::Command;

pub struct TxProcessor {
    accounts: HashMap<u16, Sender<AccountCommand>>
}

impl TxProcessor {
    pub fn handle(&mut self, command: Command) {}
}