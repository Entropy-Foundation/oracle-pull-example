use aptos_types::transaction::SignedTransaction;
use serde::{Deserialize, Serialize};

/// Request for /get_proof rest api
#[derive(Serialize, Deserialize, Debug)]
pub struct PullSupraRequest {
    pub pair_indexes: Vec<u32>,
    pub chain_type: String,
}

/// Response format for aptos based chains for /get_proof rest api
#[derive(Serialize, Deserialize, Debug)]
pub struct PullResponseSupra {
    pub pair_indexes: Vec<u32>,
    pub dkg_object: String,
    pub oracle_holder_object: String,
    pub proof_bytes: String,
}
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