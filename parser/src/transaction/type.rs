//! The `TransactionType` enum represents the type of financial transaction.
//!
//! Variants:
//! - `Deposit`: A transaction involving adding funds.
//! - `Withdrawal`: A transaction involving removing funds.
//! - `Transfer`: A transaction involving transferring funds between accounts.
//!
//! This enum provides functionality to parse transaction types from strings,
//! convert them to/from string representations, and define integer-based
//! representations.

use crate::error::ParserError;
use std::fmt::Display;
use std::str::FromStr;

/// The `TransactionType` enum represents the type of financial transaction.
#[derive(PartialEq, Eq)]
pub enum TransactionType {
    /// A transaction involving adding funds.
    Deposit,
    /// A transaction involving removing funds.
    Withdrawal,
    /// A transaction involving transferring funds between accounts.
    Transfer,
}

impl FromStr for TransactionType {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "WITHDRAWAL" => Ok(TransactionType::Withdrawal),
            "TRANSFER" => Ok(TransactionType::Transfer),
            _ => Err(ParserError::UnknownTransactionType(s.to_string())),
        }
    }
}

impl TryFrom<&str> for TransactionType {
    type Error = ParserError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "DEPOSIT"),
            TransactionType::Withdrawal => write!(f, "WITHDRAWAL"),
            TransactionType::Transfer => write!(f, "TRANSFER"),
        }
    }
}

impl TransactionType {
    fn from_repr(value: usize) -> Option<TransactionType> {
        match value {
            0 => Some(TransactionType::Deposit),
            1 => Some(TransactionType::Withdrawal),
            2 => Some(TransactionType::Transfer),
            _ => None,
        }
    }
}
