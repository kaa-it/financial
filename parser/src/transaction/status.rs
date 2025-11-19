//! The `TransactionStatus` enum represents the status of a financial transaction.
//!
//! Variants:
//! - `Success`: A successful transaction.
//! - `Failure`: A failed transaction.
//! - `Pending`: A pending transaction.
//!
//! This enum provides functionality to parse transaction statuses from strings,
//! convert them to/from string representations, and define integer-based
//! representations.
use crate::error::ParserError;
use std::fmt::Display;
use std::str::FromStr;

/// The `TransactionStatus` enum represents the status of a financial transaction.
#[derive(PartialEq, Eq, Copy, Clone, serde::Serialize, serde::Deserialize, Debug)]
pub enum TransactionStatus {
    /// A successful transaction.
    #[serde(rename = "SUCCESS")]
    Success,
    /// A failed transaction.
    #[serde(rename = "FAILURE")]
    Failure,
    /// A pending transaction.
    #[serde(rename = "PENDING")]
    Pending,
}

impl FromStr for TransactionStatus {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(TransactionStatus::Success),
            "FAILURE" => Ok(TransactionStatus::Failure),
            "PENDING" => Ok(TransactionStatus::Pending),
            _ => Err(ParserError::UnknownTransactionStatus(s.to_string())),
        }
    }
}

impl TryFrom<&str> for TransactionStatus {
    type Error = ParserError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Success => write!(f, "SUCCESS"),
            TransactionStatus::Failure => write!(f, "FAILURE"),
            TransactionStatus::Pending => write!(f, "PENDING"),
        }
    }
}

impl TransactionStatus {
    pub fn from_repr(value: usize) -> Option<TransactionStatus> {
        match value {
            0 => Some(TransactionStatus::Success),
            1 => Some(TransactionStatus::Failure),
            2 => Some(TransactionStatus::Pending),
            _ => None,
        }
    }
}
