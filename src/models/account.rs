use anyhow::Result;
use serde::Serialize;

use crate::errors::TransactionError;

#[derive(Serialize, Clone, Debug)]
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

    pub fn get_client(&self) -> u16 { self.client }

    pub fn get_available(&self) -> f32 { self.available }

    pub fn get_held(&self) -> f32 { self.held }

    pub fn get_total(&self) -> f32 { self.total }

    pub fn get_locked(&self) -> bool { self.locked }

    pub fn deposit(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        self.available += amount;
        self.total += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        if amount > self.available {
            return Err(TransactionError::WithdrawError.into());
        }
        self.available -= amount;
        self.total -= amount;
        Ok(())
    }

    pub fn held(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        if amount > self.available {
            return Err(TransactionError::HeldError.into());
        }
        self.available -= amount;
        self.held += amount;
        Ok(())
    }

    pub fn free(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        if amount > self.held {
            return Err(TransactionError::UnheldError.into());
        }
        self.held -= amount;
        self.available += amount;
        Ok(())
    }

    pub fn chargeback(&mut self, amount: f32) -> Result<()> {
        self.assert_lock()?;
        if amount > self.held {
            return Err(TransactionError::UnheldError.into());
        }
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
        Ok(())
    }

    fn assert_lock(&self) -> Result<()> {
        if self.locked {
            Err(TransactionError::AccountLocked(self.client).into())
        } else {
            Ok(())
        }
    }
}
