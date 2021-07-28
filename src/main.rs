mod errors;
mod models;

use anyhow::Result;
use models::{account::Account, transaction::Transaction};
use std::{collections::HashMap, env};
use tokio::fs::File;
use tokio_stream::StreamExt;

type TransactionsMap = HashMap<u32, Transaction>;
type AccountsMap = HashMap<u16, Account>;

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = env::args().nth(1).expect("CSV path required");
    let mut rdr = csv_async::AsyncDeserializer::from_reader(File::open(file_path).await?);
    let mut records = rdr.deserialize::<Transaction>();

    while let Some(record) = records.next().await {
        let transaction = record?;
        println!("{:#?}", transaction);
    }

    Ok(())
}
