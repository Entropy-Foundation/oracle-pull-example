use aptos::common::types::CliError;
use aptos_sdk::move_types::account_address::AccountAddressParseError;
use thiserror::Error;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ConnectorError {
    #[error("invalid url")]
    InvalidUrl,
    #[error(transparent)]
    MoveAccountAddressParse(#[from] AccountAddressParseError),
    #[error(transparent)]
    Cli(#[from] CliError),
}
