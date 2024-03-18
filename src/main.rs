mod cli;
mod parser;
mod transactions_processor;
pub mod types;

use cli::check_command_and_get_file_path_value;
use parser::parse_csv_data;
use transactions_processor::process_transactions;
use types::{Account, TransactionError};

use std::{fs::File, io::BufReader};

fn main() -> Result<(), TransactionError> {
    // Check command line args and get file path
    let file_path = check_command_and_get_file_path_value(None)?;

    let br = BufReader::new(
        File::open(file_path).map_err(|err| TransactionError::FileReadingError(err.to_string()))?,
    );

    // Parse buffered stream
    let (
        client_transaction_map,
        transaction_map,
        transaction_dispute_map,
        transaction_resolve_map,
        transaction_chargeback_map,
    ) = parse_csv_data(Box::new(br))?;

    let accounts_balance = process_transactions(
        client_transaction_map,
        transaction_map,
        transaction_dispute_map,
        transaction_resolve_map,
        transaction_chargeback_map,
    )?;

    // Print account balances
    print_accounts_balance(accounts_balance);

    Ok(())
}

fn print_accounts_balance(accounts_balance: Vec<Account>) {
    println!("client,available,held,total,locked");

    for account in accounts_balance {
        println!(
            "{},{:.4},{:.4},{:.4},{}",
            account.client, account.available, account.held, account.total, account.locked
        );
    }
}

// Test trasaction processor
#[cfg(test)]
mod tests {
    use crate::{
        parser::parse_csv_data, transactions_processor::process_transactions,
        types::TransactionError,
    };

    const SAMPLE_INPUT: &str = "type, client, tx, amount
    deposit, 1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0
    withdrawal, 1, 4, 1.5
    withdrawal, 2, 5, 3.0";

    const INVALID_INPUT: &str = "type, client, tx, amount
     1, 1, 1.0
    deposit, 2, 2, 2.0
    deposit, 1, 3, 2.0";

    #[test]
    fn can_parse_input_and_process_transactions() -> Result<(), TransactionError> {
        let str_buf = stringreader::StringReader::new(SAMPLE_INPUT);

        // Parse buffered stream
        let (
            client_transaction_map,
            transaction_map,
            transaction_dispute_map,
            transaction_resolve_map,
            transaction_chargeback_map,
        ) = parse_csv_data(Box::new(str_buf))?;

        let accounts_balance = process_transactions(
            client_transaction_map,
            transaction_map,
            transaction_dispute_map,
            transaction_resolve_map,
            transaction_chargeback_map,
        )?;

        // Print account balances
        crate::print_accounts_balance(accounts_balance);
        Ok(())
    }

    #[test]
    fn can_not_parse_invalid_transactions() -> Result<(), TransactionError> {
        let str_buf = stringreader::StringReader::new(INVALID_INPUT);

        // Parse buffered stream
        match parse_csv_data(Box::new(str_buf)) {
            Err(TransactionError::InvalidTransactionRecord(_)) => Ok(()),
            _ => Err(TransactionError::UnknownError(
                "expected InvalidTransactionRecord failure but it parsed succesfully".to_string(),
            )),
        }
    }
}
