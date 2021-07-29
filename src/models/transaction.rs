use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub enum TransactionType {
    #[serde(alias = "deposit")]
    Deposit,
    #[serde(alias = "withdrawal")]
    Withdrawal,
    #[serde(alias = "dispute")]
    Dispute,
    #[serde(alias = "resolve")]
    Resolve,
    #[serde(alias = "chargeback")]
    Chargeback,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(alias = "type")]
    pub ty: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,
}

