use crate::types::TransactionError;
use clap::{Arg, Command};

/// Parse command line args for file path and returns the value
///
/// # Arguments
///
/// * `fallback` - fallback or default value to return if no value is found
///
/// # Returns
///
/// * String - parsed value of the given argument
pub fn check_command_and_get_file_path_value(
    fallback: Option<&str>,
) -> Result<String, TransactionError> {
    let arg_name = "file";
    let matches = Command::new("transaction processor")
        .version("0.0.1")
        .author("Vineet Pant")
        .about("A simple toy payments engine that reads a series of transactions from a CSV.")
        .arg(Arg::new(arg_name).required(true).help("CSV file path"))
        .get_matches();

    match matches.get_one::<String>(arg_name) {
        Some(value) => Ok(value.to_owned()),
        None => match fallback {
            Some(value) => Ok(value.to_owned()),
            None => Err(TransactionError::ArgumentMissing(format!(
                "no value for {} given",
                arg_name
            ))),
        },
    }
}
