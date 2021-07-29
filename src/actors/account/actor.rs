use anyhow::Result;

use crate::{errors::AccountError, models::{account::Account, actor::Actor, transaction::TransactionType}};
use std::{collections::HashMap};
use crate::actors::account::messages::Command;

struct TransactionData {
    pub ty: TransactionType,
    pub amount: f32,
    pub disputed: bool
}

pub struct AccountActor {
    account: Account,
    transactions: HashMap<u32, TransactionData>,
}

impl AccountActor {
    pub fn new(client_id: u16) -> Self {
        Self {
            account: Account::new(client_id),
            transactions: HashMap::new(),
        }
    }

    fn withdraw(&mut self, tx_id: u32, amount: f32) -> Result<()> {
        self.account.withdraw(amount)?;
        self.transactions.insert(tx_id, TransactionData { 
            ty: TransactionType::Withdrawal, 
            amount, 
            disputed: false 
        });
        Ok(())
    }
    
    fn deposit(&mut self, tx_id: u32, amount: f32) -> Result<()> {
        self.account.deposit(amount)?;
        self.transactions.insert(tx_id, TransactionData { 
            ty: TransactionType::Deposit, 
            amount, 
            disputed: false 
        });
        Ok(())
    }

    fn dispute(&mut self, tx_id: u32) -> Result<()> {
        let tx = self.transactions
        .get_mut(&tx_id)
        .ok_or(AccountError::TxNotFound(
            tx_id, 
            self.account.get_client()
        ))?;
        if tx.ty == TransactionType::Deposit && !tx.disputed {
            self.account.held(tx.amount)?;
            tx.disputed = true;
        }
        Ok(())
    }

    fn resolve(&mut self, tx_id: u32) -> Result<()> {
        let tx = self.transactions
        .get_mut(&tx_id)
        .ok_or(AccountError::TxNotFound(
            tx_id, 
            self.account.get_client()
        ))?;
        if tx.disputed {
            self.account.free(tx.amount)?;
            tx.disputed = false;
        }
        Ok(())
    }

    fn chargeback(&mut self, tx_id: u32) -> Result<()> {
        let tx = self.transactions
        .get_mut(&tx_id)
        .ok_or(AccountError::TxNotFound(
            tx_id, 
            self.account.get_client()
        ))?;
        if tx.disputed {
            self
            .account
            .chargeback(tx.amount)?;
            tx.disputed = false;
        }
        Ok(())
    }
}

impl Actor<Command> for AccountActor {
    type Output = ();

    fn handle(&mut self, command: Command) -> Result<Self::Output> {
        let _ = match command {
            Command::Withdraw(tx_id, amount) => self.withdraw(tx_id, amount),
            Command::Deposit(tx_id, amount) => self.deposit(tx_id, amount),
            Command::Dispute(tx_id) => self.dispute(tx_id),
            Command::Resolve(tx_id) => self.resolve(tx_id),
            Command::Chargeback(tx_id) => self.chargeback(tx_id),
            Command::Stop(rx) => {
                let _ = rx.send(self.account.clone());
                Ok(())
            },
        };
        Ok(())
    }
}