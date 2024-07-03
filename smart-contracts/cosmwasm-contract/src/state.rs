use crate::msg::OracleHolder;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub supra_pull_contract: String,
    pub owner: Addr,
    pub price_data: HashMap<u32, OracleHolder>,
}

pub const STATE: Item<State> = Item::new("state");
