use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Not enough funds to withdraw")]
    WithdrawError,
    #[error("Not enough available funds to held")]
    HeldError,
    #[error("Not enough held funds to free")]
    UnheldError,
    #[error("Account `{0}` is locked")]
    AccountLocked(u16),
}

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("Transaction `{0}` not found for client {1}")]
    TxNotFound(u32, u16)
}
