use crate::error::ParserError;
use crate::transaction_status::TransactionStatus;
use crate::transaction_type::TransactionType;

mod csv_parser;
mod error;
mod transaction_status;
mod transaction_type;

struct Transaction {
    tx_id: u64,
    tx_type: TransactionType,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: u64,
    status: TransactionStatus,
    description: String,
}

impl Transaction {
    fn new(
        id: u64,
        tx_type: TransactionType,
        from_user_id: u64,
        to_user_id: u64,
        amount: u64,
        timestamp: u64,
        status: TransactionStatus,
        description: String,
    ) -> Self {
        Self {
            tx_id: id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        }
    }
}

trait ParserFactory {
    type Parser: Parser;
    fn create_parser(&self) -> Self::Parser;
}

trait Parser {
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
