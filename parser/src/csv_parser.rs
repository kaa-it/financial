use crate::error::ParserError;
use crate::transaction_status::TransactionStatus;
use crate::transaction_type::TransactionType;
use crate::{Parser, ParserFactory, Transaction};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

const CSV_HEADER: &str =
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

pub struct CsvParserFactory;

impl ParserFactory for CsvParserFactory {
    type Parser = CsvParser;

    fn create_parser(&self) -> Self::Parser {
        CsvParser
    }
}

pub struct CsvParser;

impl Parser for CsvParser {
    fn read_from<R: Read>(&self, r: &mut R) -> Result<Vec<Transaction>, ParserError>
    where
        Self: Sized,
    {
        let reader = BufReader::new(r);

        let mut is_first_line = true;

        let mut transactions: Vec<Transaction> = vec![];

        for line in reader.lines() {
            let line = line?;

            if is_first_line {
                Self::process_header(line)?;
                is_first_line = false;
            } else {
                let transaction = Self::process_transaction(line)?;
                transactions.push(transaction);
            }
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(
        &self,
        writer: &mut W,
        transactions: &Vec<Transaction>,
    ) -> Result<(), ParserError> {
        let mut buf_writer = BufWriter::new(writer);
        let header = format!("{}\n", CSV_HEADER);
        buf_writer.write_all(header.as_bytes())?;
        for transaction in transactions {
            let line = Self::serialize_transaction(transaction);
            let line = format!("{}\n", line);
            buf_writer.write_all(line.as_bytes())?;
        }

        buf_writer.flush()?;

        Ok(())
    }
}

impl CsvParser {
    fn process_header(line: String) -> Result<(), ParserError> {
        if line != CSV_HEADER {
            return Err(ParserError::InvalidCsvHeader(line));
        }
        Ok(())
    }

    fn process_transaction(line: String) -> Result<Transaction, ParserError> {
        let parts = &line.split(',').collect::<Vec<&str>>();

        if parts.len() != 8 {
            return Err(ParserError::InvalidCsvFormat(line));
        }

        Ok(Transaction::new(
            parts[0]
                .parse::<u64>()
                .map_err(|_| ParserError::InvalidCsvFormat(parts[0].to_string()))?,
            parts[1]
                .parse::<TransactionType>()
                .map_err(|_| ParserError::InvalidCsvFormat(parts[1].to_string()))?,
            parts[2]
                .parse::<u64>()
                .map_err(|_| ParserError::InvalidCsvFormat(parts[2].to_string()))?,
            parts[3]
                .parse::<u64>()
                .map_err(|_| ParserError::InvalidCsvFormat(parts[3].to_string()))?,
            parts[4]
                .parse::<u64>()
                .map_err(|_| ParserError::InvalidCsvFormat(parts[4].to_string()))?,
            parts[5]
                .parse::<u64>()
                .map_err(|_| ParserError::InvalidCsvFormat(parts[5].to_string()))?,
            parts[6]
                .parse::<TransactionStatus>()
                .map_err(|_| ParserError::InvalidCsvFormat(parts[6].to_string()))?,
            parts[7].to_string(),
        ))
    }

    fn serialize_transaction(transaction: &Transaction) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            transaction.tx_id,
            transaction.tx_type,
            transaction.from_user_id,
            transaction.to_user_id,
            transaction.amount,
            transaction.timestamp,
            transaction.status,
            transaction.description
        )
    }
}

mod tests {
    use super::*;
    use std::env;
    use std::fs::{File, OpenOptions};

    #[test]
    fn it_works() {
        println!("{}", env::current_dir().unwrap().display());
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
                .is_err_and(|e| matches!(e, ParserError::InvalidCsvHeader(_)))
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
