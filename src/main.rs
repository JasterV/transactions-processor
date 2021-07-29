mod errors;
mod models;
mod actors;
mod messages;

use actix::{Addr, Actor};
use anyhow::Result;
use std::{collections::HashMap, env};
use tokio::{fs::File};
use tokio_stream::StreamExt;
use crate::{messages::{Command, Stop}, models::{account::Account, transaction::Transaction}};
use actors::account::AccountActor;

type AccountActors = HashMap<u16, Addr<AccountActor>>;

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = env::args().nth(1).expect("CSV path required");
    let mut rdr = csv_async::AsyncDeserializer::from_reader(File::open(file_path).await?);
    let mut records = rdr.deserialize::<Transaction>();

    let mut addr_map: AccountActors = HashMap::new();

    while let Some(record) = records.next().await {
        let transaction = record?;

        if addr_map.contains_key(&transaction.client) {
           addr_map.get(&transaction.client).unwrap().do_send::<Command>(transaction.into());
        } else {
            let actor = AccountActor::new(transaction.client);
            let addr = actor.start();
            addr_map.insert(transaction.client, addr.clone());
            addr.do_send::<Command>(transaction.into());
        }
    }
    
    fetch_and_display_accounts(&addr_map).await?;
    Ok(())
}

async fn fetch_and_display_accounts(actors: &AccountActors) -> Result<()> {
    println!("client,available,held,total,locked");
    for addr in actors.values() {
        let account: Account = addr.send(Stop).await?;
        println!(
            "{},{},{},{},{}", 
            account.get_client(), 
            account.get_available(), 
            account.get_held(),
            account.get_total(),
            account.get_locked()
        )  
    }
    Ok(())
}
