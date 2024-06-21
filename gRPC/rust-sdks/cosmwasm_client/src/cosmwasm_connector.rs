use crate::pull_service::PullResponseCosmWasm;
use cosmrs::cosmwasm::MsgExecuteContract;
use cosmrs::rpc::Client;
use cosmrs::{
    crypto::secp256k1,
    rpc,
    tx::{self, AccountNumber, Fee, Msg, SignDoc, SignerInfo},
    AccountId, Coin,
};
use serde_json::json;
use std::str::FromStr;

pub async fn invoke_cosmwasm_chain(cosmwasm: PullResponseCosmWasm) {
    const CHAIN_ID: &str = "osmo-test-5";
    const ACCOUNT_NUMBER: AccountNumber = 25903;
    const ACCOUNT_PREFIX: &str = "osmo";
    const DENOM: &str = "uosmo";
    const COIN_AMOUNT: u128 = 10000;
    const GAS_LIMIT: u64 = 900000;
    let sequence_number = 107;

    let rpc_url = "<RPC URL>"; // Rpc url for desired chain "https://rpc.testnet.osmosis.zone:443"
    let secret_key = "<PRIVATE KEY>"; // Your Private Key
    let contract_address = "<CONTRACT ADDRESS>"; // Address of your smart contract

    let sender_private_key =
        secp256k1::SigningKey::from_slice(hex::decode(secret_key).unwrap().as_slice()).unwrap();

    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(ACCOUNT_PREFIX).unwrap();
    println!("osmosis account: {:?}", sender_account_id);

    let rpc_client = rpc::client::HttpClient::new(rpc_url).unwrap();
    println!("{:?}", rpc_client.abci_info().await);

    let contract_account_id = AccountId::from_str(contract_address).unwrap();
    let execute_msg = json!({
        "verify_oracle_proof": {
            "bytes_proof" : cosmwasm.proof_bytes,
        }
    });

    let contract_msg = MsgExecuteContract {
        sender: sender_account_id.clone(),
        contract: contract_account_id,
        msg: serde_json::to_vec(&execute_msg).unwrap(),
        funds: vec![],
    };
    let tx_body = tx::BodyBuilder::new()
        .msg(contract_msg.to_any().unwrap())
        .finish();

    let fee = Fee {
        amount: vec![Coin {
            amount: COIN_AMOUNT,
            denom: DENOM.parse().unwrap(),
        }],
        gas_limit: GAS_LIMIT,
        payer: Some(sender_account_id),
        granter: None,
    };
    let auth_info =
        SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);
    let sign_doc = SignDoc::new(
        &tx_body,
        &auth_info,
        &CHAIN_ID.parse().unwrap(),
        ACCOUNT_NUMBER,
    )
    .unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();
    let tx_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();
    println!("tx_response:{:?}", tx_response);
}
