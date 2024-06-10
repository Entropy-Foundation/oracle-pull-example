use serde::{Deserialize, Serialize};

/// Request for /get_proof rest api
#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequest {
    pub pair_indexes: Vec<u32>,
    pub chain_type: String,
}

/// Response format for evm based chains for /get_proof rest api
#[derive(Serialize, Deserialize, Debug)]
pub struct PullResponseEvm {
    pub pair_indexes: Vec<u32>,
    pub proof_bytes: String,
}
