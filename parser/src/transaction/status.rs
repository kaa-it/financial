use crate::error::ParserError;
use std::fmt::Display;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
pub enum TransactionStatus {
    Success,
    Failure,
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
    fn from_repr(value: usize) -> Option<TransactionStatus> {
        match value {
            0 => Some(TransactionStatus::Success),
            1 => Some(TransactionStatus::Failure),
            2 => Some(TransactionStatus::Pending),
            _ => None,
        }
    }
}
