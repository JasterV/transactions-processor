#[macro_use]
extern crate async_trait;

mod actors;
mod errors;
mod models;

use actors::{
    run_async_actor,
    tx_processor::{actor::TxProcessor, messages::Command},
};
use anyhow::Result;
use csv_async::AsyncDeserializer;
use models::{account::Account, transaction::Transaction};
use std::env;
use tokio::{
    fs::File,
    sync::{mpsc::Sender, oneshot},
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let path: String = env::args().nth(1).expect("CSV path required");
    let rdr: AsyncDeserializer<File> = csv_deserializer(&path).await?;
    let addr: Sender<Command> = run_async_actor(TxProcessor::new());
    process_transactions(&addr, rdr).await?;
    let accounts: Vec<Account> = stop_processor(&addr).await?;
    Ok(display_accounts(&accounts))
}

async fn process_transactions(
    addr: &Sender<Command>,
    mut rdr: AsyncDeserializer<File>,
) -> Result<()> {
    let mut records = rdr.deserialize::<Transaction>();
    while let Some(record) = records.next().await {
        let transaction = record?;
        addr.send(Command::SendTx(transaction)).await?;
    }
    Ok(())
}

async fn csv_deserializer(path: &str) -> Result<AsyncDeserializer<File>> {
    Ok(csv_async::AsyncDeserializer::from_reader(
        File::open(path).await?,
    ))
}

async fn stop_processor(processor: &Sender<Command>) -> Result<Vec<Account>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    processor.send(Command::Stop(resp_tx)).await?;
    Ok(resp_rx.await?)
}

fn display_accounts(accounts: &[Account]) {
    println!("client,available,held,total,locked");
    for account in accounts.iter() {
        println!(
            "{},{},{},{},{}",
            account.get_client(),
            account.get_available(),
            account.get_held(),
            account.get_total(),
            account.get_locked()
        )
    }
}
