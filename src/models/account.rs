use anyhow::Result;
use serde::Serialize;

use crate::errors::TransactionError;

#[derive(Serialize, Debug)]
pub struct Account {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

impl Account {
    pub fn new(client: u16) -> Self {
        Self {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn deposit(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        self.available += amount;
        self.total += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        if amount > self.available {
            Err(TransactionError::WithdrawError)?;
        }
        self.available -= amount;
        self.total -= amount;
        Ok(())
    }

    pub fn held(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        if amount > self.available {
            Err(TransactionError::HeldError)?;
        }
        self.available -= amount;
        self.held += amount;
        Ok(())
    }

    pub fn free(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        if amount > self.held {
            Err(TransactionError::UnheldError)?;
        }
        self.held -= amount;
        self.available += amount;
        Ok(())
    }

    pub fn chargeback(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        if amount > self.held {
            Err(TransactionError::UnheldError)?;
        }
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
        Ok(())
    }

    fn assert_lock(&self) -> Result<()> {
        if self.locked {
            Err(TransactionError::AccountLocked(self.client))?
        } else {
            Ok(())
        }
    }
}
