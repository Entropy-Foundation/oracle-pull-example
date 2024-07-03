use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub supra_pull_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateSupraContract { supra_pull_contract: String },
    VerifyOracleProof { proof_bytes: Vec<u8> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get Supra Pull Contract address from the currently set address in storage
    #[returns(ContractResponse)]
    GetSupraPullContract {},
    // Get price data from pair_id within own storage
    #[returns(PriceData)]
    GetPairDataInternal { pair_id: u32 },
    #[returns(OracleHolder)]
    GetPairDataSupra { pair_id: u32 },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct ContractResponse {
    pub supra_pull_contract: String,
}

#[cw_serde]
pub struct PriceData {
    pub pair: u32,
    pub price: u128,
    pub timestamp: u64,
    pub decimal: u16,
    pub round: u64,
}

#[cw_serde]
pub struct OracleHolder {
    pub price: u128,
    pub timestamp: u64,
    pub decimal: u16,
    pub round: u64,
}
