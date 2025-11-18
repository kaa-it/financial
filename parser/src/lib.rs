use crate::error::ParserError;
mod csv_parser;
mod error;
mod transaction;
mod txt_parser;

use crate::transaction::Transaction;
pub use csv_parser::CsvParserFactory;
pub use txt_parser::TxtParserFactory;

pub trait ParserFactory {
    type Parser: Parser;
    fn create_parser(&self) -> Self::Parser;
}

pub trait Parser {
    // Парсит из любого источника, реализующего трейт Read
    fn read_from<R: std::io::Read>(&self, r: &mut R) -> Result<Vec<Transaction>, ParserError>
    where
        Self: Sized;

    // Записывает отчёт в любой приёмник, реализующий трейт Write
    fn write_to<W: std::io::Write>(
        &self,
        writer: &mut W,
        transactions: &Vec<Transaction>,
    ) -> Result<(), ParserError>;
}
