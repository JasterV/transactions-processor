use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub ty: String,
    pub client: u16,
    pub tx: u32,
    pub amount: f32,
}

#[derive(Serialize, Debug)]
pub struct TransactionResult {
    pub client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}
