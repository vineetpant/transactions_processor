// use serde::Deserialize;

use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize)]
pub enum TransactionType {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback,
}

#[derive(Deserialize)]
pub struct Transaction {
    pub r#type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,
}

pub struct Account {
    pub client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("invalid command, {0}")]
    InvalidCommand(String),
    #[error("arugment missing, {0}")]
    ArgumentMissing(String),
    #[error("invalid transaction record structure, {0}")]
    InvalidTransactionRecord(String),
    #[error("error in reading file, {0}")]
    FileReadingError(String),
    #[error("transaction processing failed, {0}")]
    TransactionProcessingFailed(String),
    #[error("unknown error, {0}")]
    UnknownError(String),
}
