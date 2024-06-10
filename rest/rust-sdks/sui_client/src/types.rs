use serde::{Deserialize, Serialize};

/// Request for /get_proof rest api
#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequest {
    pub pair_indexes: Vec<u32>,
    pub chain_type: String,
}

/// Response format for sui based chains for /get_proof rest api
#[derive(Serialize, Deserialize, Debug)]
pub struct PullResponseSui {
    pub pair_indexes: Vec<u32>,
    pub dkg_object: String,
    pub oracle_holder_object: String,
    pub merkle_root_object: String,
    pub proof_bytes: String,
}
