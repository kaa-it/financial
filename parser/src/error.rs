use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParserError {
    UnknownTransactionStatus(String),
    UnknownTransactionType(String),
    IoError(std::io::Error),
    InvalidCsvHeader(String),
    InvalidCsvFormat(String),
    InvalidTxtFormat(String),
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
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl Error for ParserError {}
