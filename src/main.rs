mod errors;
mod models;
mod actors;

use anyhow::Result;
use csv_async::DeserializeRecordsStream;
use std::{collections::HashMap, env};
use tokio::{fs::File, sync::{mpsc::Sender, oneshot}};
use tokio_stream::StreamExt;
use crate::{models::{account::Account, transaction::Transaction}};
use actors::{account::messages::Command, account::{actor::AccountActor}, run_actor};

type AccountActors = HashMap<u16, Sender<Command>>;

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = env::args().nth(1).expect("CSV path required");
    let mut rdr = csv_async::AsyncDeserializer::from_reader(File::open(file_path).await?);
    let records = rdr.deserialize::<Transaction>();

    let accounts = _main(records).await?;
   
    fetch_and_display_accounts(&accounts);
    Ok(())
}

async fn _main(mut records: DeserializeRecordsStream<'_, File, Transaction>) -> Result<Vec<Account>> {
    let mut addr_map: AccountActors = HashMap::new();

    while let Some(record) = records.next().await {
        let transaction = record?;
        if addr_map.contains_key(&transaction.client) {
           addr_map.get(&transaction.client).unwrap().send(transaction.into()).await?;
        } else {
            let actor = AccountActor::new(transaction.client);
            let addr = run_actor(actor);
            addr_map.insert(transaction.client, addr.clone());
            addr.send(transaction.into()).await?;
        }
    }

    let accounts: Vec<Account> = stop_actors(&addr_map).await?;
    Ok(accounts)
}

async fn stop_actors(actors: &AccountActors) -> Result<Vec<Account>> {
    let mut accounts = vec![];
    for addr in actors.values() {
        let (resp_tx, resp_rx) = oneshot::channel();
        addr.send(Command::Stop(resp_tx)).await?;
        let account = resp_rx.await;
        accounts.push(account.expect("Error fetching account from actor"));
    }
    Ok(accounts)
}

fn fetch_and_display_accounts(accounts: &[Account]) {
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
