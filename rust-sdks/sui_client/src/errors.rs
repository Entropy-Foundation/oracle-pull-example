use thiserror::Error;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ConnectorError {
    #[error("invalid url")]
    InvalidUrl,
    #[error("Error with Sui callback  err:{0}")]
    SuiTransaction(String),
    #[error("invalid secret key")]
    InvalidSecretKey,
}
