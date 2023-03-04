use std::result::Result;

use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::{errors::{AtmError, CheckingError, BankAccountError}, events::BankAccountEvent, commands::BankAccountCommand};

pub struct BankAccountServices;

impl BankAccountServices {
    async fn atm_withdrawal(&self, atm_id: &str, amount: f64) -> Result<(), AtmError> {
        Ok(())
    }

    async fn validate_check(&self, account: &str, check: &str) -> Result<(), CheckingError> {
        Ok(())
    }
}


#[derive(Deserialize, Serialize, Default)]
pub struct BankAccount {
    opened: bool,
    // don't do this in real life
    balance: f64,
}

#[async_trait::async_trait]
impl Aggregate for BankAccount {
    type Command = BankAccountCommand;
    type Event = BankAccountEvent;
    type Services = BankAccountServices;
    type Error = BankAccountError;

    // This identifier should be unique to the system
    fn aggregate_type() -> String {
        "Account".to_string()
    }

    async fn handle(&self, command: Self::Command, services: &Self::Services) -> Result<Vec<Self::Event>, Self::Error>{
        todo!()
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            BankAccountEvent::AccountOpened { .. } => { self.opened = true }
            BankAccountEvent::CustomerDepositedMoney { amount: _, balance } => {
                self.balance = balance
            }
            BankAccountEvent::CustomerWithdrewCash { amount: _, balance } => {
                self.balance = balance;
            }
            BankAccountEvent::CustomerWroteCheck { check_number: _, amount: _, balance } => {
                self.balance= balance;
            }
        }
    }
}