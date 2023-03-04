use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpened {
        account_id: String,
    },
    CustomerDepositedMoney {
        amount: f64,
        balance: f64,
    },
    CustomerWithdrewCash {
        amount: f64,
        balance: f64,
    },
    CustomerWroteCheck {
        check_number: String,
        amount: f64,
        balance: f64,
    }
}


impl cqrs_es::DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            Self::AccountOpened { .. } => "AccountOpened",
            Self::CustomerDepositedMoney { .. } => "CustomerDepositedMoney",
            Self::CustomerWithdrewCash { .. } => "CustomerWithdrewCash",
            Self::CustomerWroteCheck { .. } => "CustomerWroteCheck",
        };

        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}