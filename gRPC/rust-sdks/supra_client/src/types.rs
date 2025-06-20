use serde::{Deserialize, Serialize};
use aptos_types::transaction::SignedTransaction;

/// Transaction types for supra
#[derive(Serialize, Deserialize, Debug)]
pub enum SupraTransaction {
    Move(SignedTransaction),
}

/// Response format for `/accounts` rest api from supra
#[derive(Serialize, Deserialize, Debug)]
pub struct SupraAccountResponse {
    pub sequence_number: u64,
    pub authentication_key: String,
}