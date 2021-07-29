use crate::{actors::{account::{actor::AccountActor, messages::Command as AccountCommand}, run_actor}, models::{account::Account, actor::AsyncActor, transaction::Transaction}};
use tokio::sync::{mpsc::Sender, oneshot};
use std::collections::HashMap;
use super::messages::Command;
use anyhow::Result;

pub struct TxProcessor {
    accounts: HashMap<u16, Sender<AccountCommand>>
}

impl TxProcessor {

    pub fn new() -> Self {
        Self { accounts: HashMap::new() }
    }

    async fn send_tx(&mut self, transaction: Transaction) -> Result<()> {
        let addr = self.accounts.entry(transaction.client).or_insert_with(|| {
            let actor = AccountActor::new(transaction.client);
            run_actor(actor)
        });
        addr.send(transaction.into()).await?;
        Ok(())
    }

    async fn stop_actors(&self) -> Result<Vec<Account>> {
        let mut accounts = vec![];
        for addr in self.accounts.values() {
            let (resp_tx, resp_rx) = oneshot::channel();
            addr.send(AccountCommand::Stop(resp_tx)).await?;
            let account = resp_rx.await;
            accounts.push(account.expect("Error fetching account from actor"));
        }
        Ok(accounts)
    }
}

#[async_trait]
impl AsyncActor<Command> for TxProcessor {
    type Output = ();

    async fn handle(&mut self, command: Command) -> Result<Self::Output> {
        match command {
            Command::SendTx(transaction ) => {
                self.send_tx(transaction).await?;
            },
            Command::Stop(responder) => {
                let accounts = self.stop_actors().await?;
                let _ = responder.send(accounts);
            }
        }
        Ok(())
    }
}