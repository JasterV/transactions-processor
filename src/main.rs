#[macro_use]
extern crate async_trait;

mod errors;
mod models;
mod actors;

use anyhow::Result;
use csv_async::DeserializeRecordsStream;
use std::env;
use tokio::{fs::File, sync::{mpsc::Sender, oneshot}};
use tokio_stream::StreamExt;
use crate::{models::{account::Account, transaction::Transaction}};
use actors::{run_async_actor, tx_processor::{actor::TxProcessor, messages::Command}};

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = env::args().nth(1).expect("CSV path required");
    let mut rdr = csv_async::AsyncDeserializer::from_reader(File::open(file_path).await?);
    let records = rdr.deserialize::<Transaction>();
    _main(records).await?;
    Ok(())
}

async fn _main(mut records: DeserializeRecordsStream<'_, File, Transaction>) -> Result<()> {
    let tx_processor_addr = run_async_actor(TxProcessor::new());

    while let Some(record) = records.next().await {
        let transaction = record?;
        tx_processor_addr.send(Command::SendTx(transaction)).await?;
    }
    
    let accounts = send_stop(&tx_processor_addr).await?;
    display_accounts(&accounts);
    Ok(())
}

async fn send_stop(processor: &Sender<Command>) -> Result<Vec<Account>> {
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
