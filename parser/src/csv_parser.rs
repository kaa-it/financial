//! The `csv_parser` module provides functionality to parse financial transactions from CSV files.

use crate::error::ParserError;
use crate::transaction::Transaction;
use crate::{Parser, ParserFactory};
use std::io::{Read, Write};

/// The `CsvParserFactory` struct is a factory for creating CSV parsers.
pub struct CsvParserFactory;

impl ParserFactory for CsvParserFactory {
    type Parser = CsvParser;

    fn create_parser(&self) -> Self::Parser {
        CsvParser
    }
}

/// The `CsvParser` struct is a parser for CSV files.
pub struct CsvParser;

impl Parser for CsvParser {
    fn read_from<R: Read>(&self, r: &mut R) -> Result<Vec<Transaction>, ParserError>
    where
        Self: Sized,
    {
        let mut reader = csv::Reader::from_reader(r);

        let mut transactions: Vec<Transaction> = vec![];

        for result in reader.deserialize() {
            let transaction: Transaction = result?;
            transactions.push(transaction);
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(
        &self,
        writer: &mut W,
        transactions: &[Transaction],
    ) -> Result<(), ParserError> {
        let mut writer = csv::Writer::from_writer(writer);
        for transaction in transactions {
            writer.serialize(transaction)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, OpenOptions};

    #[test]
    fn it_works() {
        let mut file = File::open("../samples/records_example.csv").unwrap();
        let res = CsvParser.read_from(&mut file);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 1000);
    }

    #[test]
    fn it_is_invalid_csv_header() {
        let mut file = File::open("../samples/records_example_invalid_header.csv").unwrap();
        assert!(
            CsvParser
                .read_from(&mut file)
                .is_err_and(|e| matches!(e, ParserError::InvalidCsvFormat(_)))
        );
    }

    #[test]
    fn it_is_invalid_csv_format() {
        let mut file = File::open("../samples/records_example_invalid_format.csv").unwrap();
        assert!(
            CsvParser
                .read_from(&mut file)
                .is_err_and(|e| matches!(e, ParserError::InvalidCsvFormat(_)))
        );
    }

    #[test]
    fn it_writes_csv() {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open("../samples/records_example_out.csv")
            .unwrap();
        let transactions = CsvParser
            .read_from(&mut File::open("../samples/records_example.csv").unwrap())
            .unwrap();
        assert!(CsvParser.write_to(&mut file, &transactions).is_ok());
        assert!(CsvParser.read_from(&mut file).is_ok());
        std::fs::remove_file("../samples/records_example_out.csv").unwrap();
    }
}
