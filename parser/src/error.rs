//! The `ParserError` enum represents errors that can occur while parsing financial transactions.
//!
//! Variants:
//! - `UnknownTransactionStatus`: An unknown transaction status was encountered.
//! - `UnknownTransactionType`: An unknown transaction type was encountered.
//! - `IoError`: An I/O error occurred.
//! - `InvalidCsvHeader`: An invalid CSV header was encountered.
//! - `InvalidCsvFormat`: An invalid CSV format was encountered.
//! - `InvalidTxtFormat`: An invalid TXT format was encountered.

use std::error::Error;
use std::fmt::Display;

/// The `ParserError` enum represents errors that can occur while parsing financial transactions.
#[derive(Debug)]
pub enum ParserError {
    /// An unknown transaction status was encountered.
    UnknownTransactionStatus(String),
    /// An unknown transaction type was encountered.
    UnknownTransactionType(String),
    /// An I/O error occurred.
    IoError(std::io::Error),
    /// An invalid CSV header was encountered.
    InvalidCsvHeader(String),
    /// An invalid CSV format was encountered.
    InvalidCsvFormat(String),
    /// An invalid TXT format was encountered.
    InvalidTxtFormat(String),
    /// An invalid BIN format was encountered.
    InvalidBinFormat(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::UnknownTransactionStatus(status) => {
                write!(f, "Unknown transaction status: {}", status)
            }
            ParserError::UnknownTransactionType(type_) => {
                write!(f, "Unknown transaction type: {}", type_)
            }
            ParserError::IoError(e) => {
                write!(f, "IO error: {}", e)
            }
            ParserError::InvalidCsvHeader(header) => {
                write!(f, "Invalid CSV header: {}", header)
            }
            ParserError::InvalidCsvFormat(line) => {
                write!(f, "Invalid CSV format: {}", line)
            }
            ParserError::InvalidTxtFormat(line) => {
                write!(f, "Invalid TXT format: {}", line)
            }
            ParserError::InvalidBinFormat(line) => {
                write!(f, "Invalid BIN format: {}", line)
            }
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<csv::Error> for ParserError {
    fn from(e: csv::Error) -> Self {
        Self::InvalidCsvFormat(e.to_string())
    }
}

impl Error for ParserError {}
