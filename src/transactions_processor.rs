use crate::types::{Account, Transaction, TransactionError, TransactionType};
use std::collections::HashMap;

/// Process CSV transactions and return account balances
///
/// # Arguments
///
/// * HashMap<u16, Vec<u32>>, - mapping of client with transactions(deposit/withdrawal)
/// * HashMap<u32, Transaction>, - mapping of txn id with transactions(deposit/withdrawal)
/// * HashMap<u32, Transaction>, - mapping of txn id with transactions(disputed)
/// * HashMap<u32, Transaction>, - mapping of txn id with transactions(resolved)
/// * HashMap<u32, Transaction>, - mapping of txn id with transactions(chargedback)
///
/// # Returns
///
/// * Vec<Account>, - Vector of all accounts with balances
pub fn process_transactions(
    client_transaction_map: HashMap<u16, Vec<u32>>,
    transaction_map: HashMap<u32, Transaction>,
    transaction_dispute_map: HashMap<u32, Transaction>,
    transaction_resolve_map: HashMap<u32, Transaction>,
    transaction_chargeback_map: HashMap<u32, Transaction>,
) -> Result<Vec<Account>, TransactionError> {
    let mut client_accounts: Vec<Account> = vec![];

    for (client, txns) in client_transaction_map {
        let mut lock = false;
        let mut available: f32 = 0.0;
        let mut held: f32 = 0.0;
        let mut total: f32 = 0.0;

        for tx in txns {
            let transaction =
                transaction_map
                    .get(&tx)
                    .ok_or(TransactionError::TransactionProcessingFailed(
                        "Invalid transaction in transaction map".to_owned(),
                    ))?;

            let amount =
                transaction
                    .amount
                    .ok_or(TransactionError::TransactionProcessingFailed(
                        "Error in parsing transaction amount".to_owned(),
                    ))?;

            match transaction.r#type {
                TransactionType::Deposit => {
                    available = available + amount;
                    total = total + amount;
                }
                TransactionType::Withdrawal => {
                    if amount <= available {
                        // Process withdrawal only if amount is less than or equal to available balance
                        available = available - amount;
                        total = total - amount;
                    }
                }
                _ => {}
            }

            // Check for dispute
            let tx_dispute = transaction_dispute_map.get(&tx);
            match tx_dispute {
                Some(txn_dispute) => {
                    // Update client balance as per disputed transaction
                    if txn_dispute.client == client {
                        // if clients are different, ignore transaction as faulty transaction
                        held = held + amount;
                        available = available - amount;
                    }

                    // Check if disputed transaction is resolved
                    let tx_resolved = transaction_resolve_map.get(&tx);
                    match tx_resolved {
                        Some(txn_resolved) => {
                            // If transaction is resolved, update client balance
                            if txn_resolved.client == client {
                                // if clients are different, ignore transaction as faulty transaction
                                held = held - amount;
                                available = available + amount;
                            }
                        }
                        None => {}
                    }

                    // Check if disputed transaction is resolved
                    let tx_chargeback = transaction_chargeback_map.get(&tx);
                    match tx_chargeback {
                        Some(txn_chargeback) => {
                            // If transaction is chargeback, update client balance and lock account
                            if txn_chargeback.client == client {
                                // if clients are different, ignore transaction as faulty transaction
                                held = held - amount;
                                total = total - amount;
                                lock = true;
                            }
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }

        let account = Account {
            client,
            available,
            held,
            total,
            locked: lock,
        };
        client_accounts.push(account);
    }

    Ok(client_accounts)
}
