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

    pub fn get_client(&self) -> u16 {
        self.client
    }

    pub fn get_available(&self) -> f32 {
        self.available
    }

    pub fn get_held(&self) -> f32 {
        self.held
    }

    pub fn get_total(&self) -> f32 {
        self.total
    }

    pub fn get_locked(&self) -> bool {
        self.locked
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_account_deposit() -> Result<()> {
        let mut account = Account::new(1);
        assert_eq!(account.get_total(), 0.0);
        assert_eq!(account.get_available(), 0.0);
        assert_eq!(account.get_held(), 0.0);
        account.deposit(1000.0)?;
        assert_eq!(account.get_total(), 1000.0);
        assert_eq!(account.get_available(), 1000.0);
        assert_eq!(account.get_held(), 0.0);
        Ok(())
    }

    #[test]
    fn test_account_withdraw() -> Result<()> {
        let mut account = Account::new(1);
        account.deposit(1000.0)?;
        account.withdraw(250.0)?;
        assert_eq!(account.get_total(), 750.0);
        assert_eq!(account.get_available(), 750.0);
        assert_eq!(account.get_held(), 0.0);
        Ok(())
    }

    #[test]
    fn test_account_held() -> Result<()> {
        let mut account = Account::new(1);
        account.deposit(1000.0)?;
        account.held(250.0)?;
        assert_eq!(account.get_total(), 1000.0);
        assert_eq!(account.get_available(), 750.0);
        assert_eq!(account.get_held(), 250.0);
        Ok(())
    }

    #[test]
    fn test_account_free_held() -> Result<()> {
        let mut account = Account::new(1);
        account.deposit(1000.0)?;
        account.held(250.0)?;
        account.free(100.0)?;
        assert_eq!(account.get_total(), 1000.0);
        assert_eq!(account.get_available(), 850.0);
        assert_eq!(account.get_held(), 150.0);
        Ok(())
    }

    #[test]
    fn test_account_chargeback() -> Result<()> {
        let mut account = Account::new(1);
        account.deposit(1000.0)?;
        account.held(250.0)?;
        assert_eq!(account.get_locked(), false);
        account.chargeback(250.0)?;
        assert_eq!(account.get_total(), 750.0);
        assert_eq!(account.get_available(), 750.0);
        assert_eq!(account.get_held(), 0.0);
        assert_eq!(account.get_locked(), true);
        assert!(account.deposit(1.0).is_err());
        assert!(account.withdraw(1.0).is_err());
        assert!(account.free(1.0).is_err());
        assert!(account.held(1.0).is_err());
        assert!(account.chargeback(1.0).is_err());
        Ok(())
    }
}
