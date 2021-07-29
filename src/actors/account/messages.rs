use crate::{models::{account::Account, responder::Responder, transaction::{Transaction, TransactionType}}};

#[derive(Debug)]
pub enum Command {
    Withdraw(f32),
    Deposit(f32),
    Dispute(u32),
    Resolve(u32),
    Chargeback(u32),
    Stop(Responder<Account>)
}

impl From<Transaction> for Command {
    fn from(tx: Transaction) -> Self {
        match tx.ty {
            TransactionType::Chargeback => Command::Chargeback(tx.tx),
            TransactionType::Deposit => Command::Deposit(tx.amount),
            TransactionType::Withdrawal => Command::Withdraw(tx.amount),
            TransactionType::Dispute => Command::Dispute(tx.tx),
            TransactionType::Resolve => Command::Resolve(tx.tx)
        }
    }
}
