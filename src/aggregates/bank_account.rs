use std::result::Result;

use cqrs_es::{Aggregate, AggregateError};
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
        match command {
            BankAccountCommand::DepositMoney { amount } => {
                let balance = self.balance + amount;
                Ok(vec![BankAccountEvent::CustomerDepositedMoney { amount, balance }])
            },
            BankAccountCommand::WithdrawMoney { amount } => {
                let balance = self.balance - amount;
                if balance < 0_f64 {
                    return Err(BankAccountError("funds not available".to_string()))
                }
                Ok(vec![BankAccountEvent::CustomerWithdrewCash { amount, balance }])
            }
            _ => { Ok(vec![]) }
        }
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



#[cfg(test)]
mod aggregate_tests {
    use super::*;
    use cqrs_es::test::TestFramework;

    type AccountTestFramework = TestFramework<BankAccount>;

    #[test]
    fn test_deposit_money() {
        let expected = BankAccountEvent::CustomerDepositedMoney { amount: 200.0, balance: 200.0 };

        AccountTestFramework::with(BankAccountServices)
            .given_no_previous_events()
            .when(BankAccountCommand::DepositMoney { amount: 200.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdrawal_money() {
        let previous = BankAccountEvent::CustomerDepositedMoney { amount: 200.0, balance: 200.0 };
        let expected = BankAccountEvent::CustomerWithdrewCash { amount: 100.0, balance: 100.0 };

        AccountTestFramework::with(BankAccountServices)
            .given(vec![previous])
            .when(BankAccountCommand::WithdrawMoney { amount: 100.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    // test widthdraw money funds not available
    fn test_withdraw_money_funds_not_available() {
        let msg = BankAccountError("funds not available".to_string());
        
        AccountTestFramework::with(BankAccountServices)
            .given_no_previous_events()
            .when(BankAccountCommand::WithdrawMoney { amount: 200.0 })
            .then_expect_error_message(format!("{}", msg).as_str());
    }
}