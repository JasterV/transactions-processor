use crate::models::{account::Account, responder::Responder, transaction::Transaction};

#[derive(Debug)]
pub enum Command {
    SendTx(Transaction),
    Stop(Responder<Vec<Account>>)
}