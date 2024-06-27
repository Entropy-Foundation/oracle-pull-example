use thiserror::Error;

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum CosmWasmConnectorError {
    #[error("invalid grpc response")]
    InvalidGRPCResponse,
}
