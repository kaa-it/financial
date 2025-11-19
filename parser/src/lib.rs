//! The `parser` crate provides functionality to parse financial transactions from CSV and TXT files.
#![deny(unreachable_pub)]
#![warn(missing_docs)]

use crate::error::ParserError;
mod bin_parser;
mod csv_parser;
mod error;
mod transaction;
mod txt_parser;

use crate::transaction::Transaction;
pub use bin_parser::BinParserFactory;
pub use csv_parser::CsvParserFactory;
pub use txt_parser::TxtParserFactory;

/// The `ParserFactory` trait represents a factory for creating parsers.
pub trait ParserFactory {
    /// The type of the parser.
    type Parser: Parser;

    /// Creates a new parser.
    fn create_parser(&self) -> Self::Parser;
}

/// The `Parser` trait represents a parser for financial transactions.
pub trait Parser {
    /// Reads transactions from a reader.
    fn read_from<R: std::io::Read>(&self, r: &mut R) -> Result<Vec<Transaction>, ParserError>
    where
        Self: Sized;

    /// Writes transactions to a writer.
    fn write_to<W: std::io::Write>(
        &self,
        writer: &mut W,
        transactions: &[Transaction],
    ) -> Result<(), ParserError>;
}
