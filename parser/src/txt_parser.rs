use crate::error::ParserError;
use crate::transaction::{Transaction, TransactionStatus, TransactionType};
use crate::{Parser, ParserFactory};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub struct TxtParserFactory;

impl ParserFactory for TxtParserFactory {
    type Parser = TxtParser;

    fn create_parser(&self) -> Self::Parser {
        TxtParser
    }
}

pub struct TxtParser;

impl Parser for TxtParser {
    fn read_from<R: Read>(&self, r: &mut R) -> Result<Vec<Transaction>, ParserError>
    where
        Self: Sized,
    {
        let reader = BufReader::new(r);
        let mut transactions: Vec<Transaction> = vec![];

        let mut transaction = Transaction::default();

        for line in reader.lines() {
            let line = line?;

            if line.starts_with("#") {
                continue;
            }

            if line.trim().is_empty() {
                transactions.push(transaction);
                transaction = Transaction::default();
                continue;
            }

            TxtParser::process_line(&mut transaction, line)?;
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(
        &self,
        writer: &mut W,
        transactions: &Vec<Transaction>,
    ) -> Result<(), ParserError> {
        let mut buf_writer = BufWriter::new(writer);
        let mut current = 1;
        for transaction in transactions {
            let line = TxtParser::serialize_transaction(transaction, current);
            current += 1;
            let line = format!("{}\n\n", line);
            buf_writer.write_all(line.as_bytes())?;
        }
        buf_writer.flush()?;
        Ok(())
    }
}

impl TxtParser {
    fn process_line(transaction: &mut Transaction, line: String) -> Result<(), ParserError> {
        let parts = line
            .split(':')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        match parts[0] {
            "TX_ID" => {
                transaction.tx_id = parts[1]
                    .parse::<u64>()
                    .map_err(|_| ParserError::InvalidTxtFormat(line))?;
            }
            "TX_TYPE" => {
                transaction.tx_type = parts[1]
                    .parse::<TransactionType>()
                    .map_err(|_| ParserError::InvalidTxtFormat(line))?;
            }
            "FROM_USER_ID" => {
                transaction.from_user_id = parts[1]
                    .parse::<u64>()
                    .map_err(|_| ParserError::InvalidTxtFormat(line))?;
            }
            "TO_USER_ID" => {
                transaction.to_user_id = parts[1]
                    .parse::<u64>()
                    .map_err(|_| ParserError::InvalidTxtFormat(line))?;
            }
            "AMOUNT" => {
                transaction.amount = parts[1]
                    .parse::<u64>()
                    .map_err(|_| ParserError::InvalidTxtFormat(line))?;
            }
            "TIMESTAMP" => {
                transaction.timestamp = parts[1]
                    .parse::<u64>()
                    .map_err(|_| ParserError::InvalidTxtFormat(line))?;
            }
            "STATUS" => {
                transaction.status = parts[1]
                    .parse::<TransactionStatus>()
                    .map_err(|_| ParserError::InvalidTxtFormat(line))?;
            }
            "DESCRIPTION" => {
                transaction.description = parts[1].to_string();
            }
            _ => {
                return Err(ParserError::InvalidTxtFormat(line));
            }
        }

        Ok(())
    }

    fn serialize_transaction(transaction: &Transaction, current: u64) -> String {
        let header = format!("# Record number {} ({})", current, transaction.tx_type);
        format!("{}\n{}", header, transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, OpenOptions};

    #[test]
    fn it_works() {
        let mut file = File::open("../samples/records_example.txt").unwrap();
        let res = TxtParser.read_from(&mut file);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 1000);
    }

    #[test]
    fn it_writes_txt() {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open("../samples/records_example_out.txt")
            .unwrap();
        let transactions = TxtParser
            .read_from(&mut File::open("../samples/records_example.txt").unwrap())
            .unwrap();
        assert!(TxtParser.write_to(&mut file, &transactions).is_ok());
        assert!(TxtParser.read_from(&mut file).is_ok());
        std::fs::remove_file("../samples/records_example_out.txt").unwrap();
    }
}
