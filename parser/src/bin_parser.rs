//! The `bin_parser` module provides functionality to parse financial transactions from BIN files.

use crate::error::ParserError;
use crate::error::ParserError::InvalidBinFormat;
use crate::transaction::{Transaction, TransactionStatus, TransactionType};
use crate::{Parser, ParserFactory};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::ErrorKind::UnexpectedEof;
use std::io::{Read, Write};

/// The magic sequence for the BIN file.
const MAGIC: &str = "YPBN";

/// The `BinParserFactory` struct is a factory for creating BIN parsers.
pub struct BinParserFactory;

impl ParserFactory for BinParserFactory {
    type Parser = BinParser;

    fn create_parser(&self) -> Self::Parser {
        BinParser
    }
}

/// The `BinParser` struct is a parser for BIN files.
pub struct BinParser;

impl Parser for BinParser {
    fn read_from<R: Read>(&self, r: &mut R) -> Result<Vec<Transaction>, ParserError>
    where
        Self: Sized,
    {
        let mut magic = [0x00; 4];

        let mut transactions: Vec<Transaction> = vec![];

        loop {
            let result = r.read_exact(&mut magic);

            if let Err(e) = result {
                if e.kind() == UnexpectedEof {
                    break;
                }
                return Err(e.into());
            }

            if String::from_utf8_lossy(&magic) != MAGIC {
                return Err(InvalidBinFormat("Wrong MAGIC sequence".to_string()));
            }

            let _ = r.read_u32::<BigEndian>()?;

            let transaction = BinParser::process_transaction(r)?;

            transactions.push(transaction);
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(
        &self,
        writer: &mut W,
        transactions: &[Transaction],
    ) -> Result<(), ParserError> {
        for transaction in transactions {
            Self::serialize_transaction(transaction, writer)?;
        }

        Ok(())
    }
}

impl BinParser {
    fn process_transaction<R: Read>(r: &mut R) -> Result<Transaction, ParserError> {
        let mut transaction = Transaction::default();

        transaction.tx_id = r.read_u64::<BigEndian>()?;
        transaction.tx_type = TransactionType::from_repr((r.read_u8()?) as usize)
            .ok_or(InvalidBinFormat("Wrong transaction type".to_string()))?;
        transaction.from_user_id = r.read_u64::<BigEndian>()?;
        transaction.to_user_id = r.read_u64::<BigEndian>()?;
        transaction.amount = r.read_u64::<BigEndian>()?;
        transaction.timestamp = r.read_u64::<BigEndian>()?;
        transaction.status = TransactionStatus::from_repr((r.read_u8()?) as usize)
            .ok_or(InvalidBinFormat("Wrong transaction status".to_string()))?;
        let description_length = r.read_u32::<BigEndian>()?;
        let mut description = vec![0x00; description_length as usize];
        r.read_exact(&mut description)?;
        let description = String::from_utf8(description)
            .map_err(|_| InvalidBinFormat("Wrong description".to_string()))?;
        transaction.description = description;

        Ok(transaction)
    }

    fn serialize_transaction<W: Write>(
        transaction: &Transaction,
        writer: &mut W,
    ) -> Result<(), ParserError> {
        let record_size = 42 + transaction.description.len();
        writer.write_all(MAGIC.as_bytes())?;
        writer.write_u32::<BigEndian>(record_size as u32)?;
        writer.write_u64::<BigEndian>(transaction.tx_id)?;
        writer.write_u8(transaction.tx_type as u8)?;
        writer.write_u64::<BigEndian>(transaction.from_user_id)?;
        writer.write_u64::<BigEndian>(transaction.to_user_id)?;
        writer.write_u64::<BigEndian>(transaction.amount)?;
        writer.write_u64::<BigEndian>(transaction.timestamp)?;
        writer.write_u8(transaction.status as u8)?;
        writer.write_all(transaction.description.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, OpenOptions};

    #[test]
    fn it_works() {
        let mut file = File::open("../samples/records_example.bin").unwrap();
        let res = BinParser.read_from(&mut file);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 1000);
    }

    #[test]
    fn it_writes_bin() {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open("../samples/records_example_out.bin")
            .unwrap();
        let transactions = BinParser
            .read_from(&mut File::open("../samples/records_example.bin").unwrap())
            .unwrap();
        assert!(BinParser.write_to(&mut file, &transactions).is_ok());
        assert!(BinParser.read_from(&mut file).is_ok());
        std::fs::remove_file("../samples/records_example_out.bin").unwrap();
    }
}
