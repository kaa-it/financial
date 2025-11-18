//! The `Transaction` struct represents a financial transaction.
//!
//! Fields:
//! - `tx_id`: The ID of the transaction.
//! - `tx_type`: The type of the transaction.
//! - `from_user_id`: The ID of the user who is sending the transaction.
//! - `to_user_id`: The ID of the user who is receiving the transaction.
//! - `amount`: The amount of the transaction.
//! - `timestamp`: The timestamp of the transaction.
//! - `status`: The status of the transaction.
//! - `description`: The description of the transaction.

mod status;
mod r#type;

pub use status::TransactionStatus;
use std::fmt::{Display, Formatter};
pub use r#type::TransactionType;

/// The `Transaction` struct represents a financial transaction.
#[derive(PartialEq, Eq)]
pub struct Transaction {
    /// The ID of the transaction.
    pub tx_id: u64,
    /// The type of the transaction.
    pub tx_type: TransactionType,
    /// The ID of the user who is sending the transaction.
    pub from_user_id: u64,
    /// The ID of the user who is receiving the transaction.
    pub to_user_id: u64,
    /// The amount of the transaction.
    pub amount: u64,
    /// The timestamp of the transaction.
    pub timestamp: u64,
    /// The status of the transaction.
    pub status: TransactionStatus,
    /// The description of the transaction.
    pub description: String,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            tx_id: 0,
            tx_type: TransactionType::Deposit,
            from_user_id: 0,
            to_user_id: 0,
            amount: 0,
            timestamp: 0,
            status: TransactionStatus::Success,
            description: "".to_string(),
        }
    }
}

impl Transaction {
    pub fn new(
        id: u64,
        tx_type: TransactionType,
        from_user_id: u64,
        to_user_id: u64,
        amount: u64,
        timestamp: u64,
        status: TransactionStatus,
        description: String,
    ) -> Self {
        Self {
            tx_id: id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TX_ID: {}\nTX_TYPE: {}\nFROM_USER_ID: {}\nTO_USER_ID: {}\nAMOUNT: {}\nTIMESTAMP: {}\nSTATUS: {}\nDESCRIPTION: {}",
            self.tx_id,
            self.tx_type,
            self.from_user_id,
            self.to_user_id,
            self.amount,
            self.timestamp,
            self.status,
            self.description
        )
    }
}
