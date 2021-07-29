use crate::models::{account::Account, responder::Responder, transaction::Transaction};

#[derive(Debug)]
pub enum Command {
    SendTx {
        client: u16,
        transaction: Transaction
    },
    Stop(Responder<Vec<Account>>)
}