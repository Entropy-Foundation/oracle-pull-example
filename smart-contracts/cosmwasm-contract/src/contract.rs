use crate::error::{ContractError, ParseReplyError};
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, OracleHolder, PriceData, QueryMsg};
use crate::state::{State, STATE};
use anybuf::Bufany;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg,
};
use cw2::set_contract_version;
use serde_json::json;
use std::collections::HashMap;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:osmo";
const CONTRACT_VERSION: &str = "0.1.1";
const VERIFY_PROOF_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        supra_pull_contract: msg.supra_pull_contract.clone(),
        owner: info.sender.clone(),
        price_data: HashMap::new(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("supra_pull_contract", msg.supra_pull_contract.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateSupraContract {
            supra_pull_contract,
        } => update_supra_contract(deps, supra_pull_contract),
        ExecuteMsg::VerifyOracleProof { proof_bytes } => {
            client_verify_oracle_proof(deps, proof_bytes)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        VERIFY_PROOF_REPLY_ID => handle_verify_proof_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_verify_proof_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let price_data_bytes = &msg
        .result
        .unwrap()
        .data
        .ok_or_else(|| ParseReplyError::ParseFailure("Missing reply data".to_owned()))
        .unwrap();

    let deserialized_price_data = Bufany::deserialize(price_data_bytes).unwrap();
    //We have used the first index value from the Buf
    let price_data: Vec<PriceData> =
        bcs::from_bytes(&deserialized_price_data.bytes(1).unwrap()).unwrap();
    STATE
        .update(deps.storage, |mut state| -> Result<_, ContractError> {
            for data in price_data.clone() {
                if let Some(oracle_holder) = state.price_data.get_mut(&data.pair) {
                    oracle_holder.price = data.price;
                    oracle_holder.round = data.round;
                    oracle_holder.timestamp = data.timestamp;
                    oracle_holder.decimal = data.decimal;
                } else {
                    state.price_data.insert(
                        data.pair,
                        OracleHolder {
                            price: data.price,
                            timestamp: data.timestamp,
                            decimal: data.decimal,
                            round: data.round,
                        },
                    );
                }
            }
            Ok(state)
        })
        .expect("State Update Panic");

    let price_data_string = serde_json::to_string(&price_data).unwrap();
    Ok(Response::default().add_attribute("client_data", price_data_string))
}

fn update_supra_contract(
    deps: DepsMut,
    supra_pull_contract: String,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.supra_pull_contract = supra_pull_contract;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "update_supra_contract"))
}

fn client_verify_oracle_proof(
    deps: DepsMut,
    proof_bytes: Vec<u8>,
) -> Result<Response, ContractError> {
    let execute_msg = json!({
        "verify_oracle_proof": {
            "bytes_proof" : proof_bytes
        }
    });
    let verify_oracle_proof = cosmwasm_std::WasmMsg::Execute {
        contract_addr: query_supra_pull_contract(deps.as_ref())
            .unwrap()
            .supra_pull_contract,
        msg: Binary::from(serde_json::to_vec(&execute_msg).unwrap()),
        funds: vec![],
    };

    // Creating a submessage that wraps the message above
    let submessage = SubMsg::reply_on_success(verify_oracle_proof, VERIFY_PROOF_REPLY_ID);
    // Creating a response with the submessage
    let response = Response::new().add_submessage(submessage);
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSupraPullContract {} => to_json_binary(&query_supra_pull_contract(deps)?),
        QueryMsg::GetPairDataInternal { pair_id } => {
            to_json_binary(&query_pair_data_internal(deps, pair_id)?)
        }
        QueryMsg::GetPairDataSupra { pair_id } => {
            to_json_binary(&query_pair_data_supra(deps, pair_id)?)
        }
    }
}

fn query_supra_pull_contract(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse {
        supra_pull_contract: state.supra_pull_contract,
    })
}
fn query_pair_data_supra(deps: Deps, pair_id: u32) -> StdResult<OracleHolder> {
    let query_message = json!({
        "get_svalue" : {
            "pair_index" : pair_id
        }
    });
    let state = STATE.load(deps.storage)?;

    let response: StdResult<OracleHolder> = deps
        .querier
        .query_wasm_smart(state.supra_pull_contract, &query_message);
    response
}

fn query_pair_data_internal(deps: Deps, pair_id: u32) -> StdResult<OracleHolder> {
    let state = STATE.load(deps.storage)?;
    if let Some(oracle_holder) = state.price_data.get(&pair_id) {
        Ok(oracle_holder.clone())
    } else {
        panic!("Pair id not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            supra_pull_contract: "Test".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetSupraPullContract {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!("Test".to_string(), value.supra_pull_contract);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            supra_pull_contract: "Test".to_string(),
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::UpdateSupraContract {
            supra_pull_contract: "Test Update".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetSupraPullContract {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!("Test Update".to_string(), value.supra_pull_contract);
    }
}
