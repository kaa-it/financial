mod status;
mod r#type;

pub use status::TransactionStatus;
use std::fmt::{Display, Formatter};
pub use r#type::TransactionType;

#[derive(PartialEq, Eq)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub description: String,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            tx_id: 0,
            tx_type: TransactionType::Deposit,
            from_user_id: 0,
            to_user_id: 0,
            amount: 0,
            timestamp: 0,
            status: TransactionStatus::Success,
            description: "".to_string(),
        }
    }
}

impl Transaction {
    pub fn new(
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

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TX_ID: {}\nTX_TYPE: {}\nFROM_USER_ID: {}\nTO_USER_ID: {}\nAMOUNT: {}\nTIMESTAMP: {}\nSTATUS: {}\nDESCRIPTION: {}",
            self.tx_id,
            self.tx_type,
            self.from_user_id,
            self.to_user_id,
            self.amount,
            self.timestamp,
            self.status,
            self.description
        )
    }
}
