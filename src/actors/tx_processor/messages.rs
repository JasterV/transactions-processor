use crate::models::{account::Account, transaction::Transaction};
use tokio::sync::oneshot::Sender as Responder;

#[derive(Debug)]
pub enum Command {
    SendTx(Transaction),
    Stop(Responder<Vec<Account>>),
}
