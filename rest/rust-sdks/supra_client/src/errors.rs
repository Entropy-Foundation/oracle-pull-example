use aptos::common::types::CliError;
use aptos_sdk::crypto::CryptoMaterialError;
use aptos_sdk::move_types::account_address::AccountAddressParseError;
use std::num::ParseIntError;
use thiserror::Error;
use url::ParseError;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ConnectorError {
    #[error("invalid url")]
    InvalidUrl,
    #[error(transparent)]
    MoveAccountAddressParse(#[from] AccountAddressParseError),
    #[error(transparent)]
    Cli(#[from] CliError),
    #[error(transparent)]
    FromHex(#[from] hex::FromHexError),
    #[error(transparent)]
    Signature(#[from] ed25519_dalek::SignatureError),
    #[error(transparent)]
    CryptoMaterial(#[from] CryptoMaterialError),
    #[error("Error with Supra callback  err:{0}")]
    SupraTransaction(String),
    #[error("Error while sending transaction to supra: {0}")]
    SupraExecuteTransactionError(#[from] reqwest::Error),
    #[error(transparent)]
    UrlParserError(#[from] ParseError),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}
