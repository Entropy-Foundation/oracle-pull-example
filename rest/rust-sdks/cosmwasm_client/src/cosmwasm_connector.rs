use crate::errors::CosmWasmConnectorError;
use crate::PullResponseCosmWasm;
use cosmrs::cosmwasm::MsgExecuteContract;
use cosmrs::proto::cosmos::auth::v1beta1::query_client::QueryClient;
use cosmrs::proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use cosmrs::proto::traits::Message;
use cosmrs::{
    crypto::secp256k1,
    rpc,
    tx::{self, Fee, Msg, SignDoc, SignerInfo},
    AccountId, Coin,
};
use serde_json::json;
use std::str::FromStr;

pub async fn invoke_cosmwasm_chain(cosmwasm: PullResponseCosmWasm) {
    const CHAIN_ID: &str = "osmo-test-5";
    const ACCOUNT_PREFIX: &str = "osmo";
    const DENOM: &str = "uosmo";
    const COIN_AMOUNT: u128 = 10000;
    const GAS_LIMIT: u64 = 900000;

    let rpc_url = "<RPC URL>"; // Rpc url for desired chain
    let grpc_url = "<GRPC URL>"; //Enter the GRPC URL of the network
    let secret_key = "<PRIVATE KEY>"; // Your Private Key
    let contract_address = "<CONTRACT ADDRESS>"; // Address of your smart contract

    let sender_private_key =
        secp256k1::SigningKey::from_slice(hex::decode(secret_key).unwrap().as_slice()).unwrap();

    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(ACCOUNT_PREFIX).unwrap();
    println!("osmosis account: {:?}", sender_account_id);

    let (account_number, sequence_number) =
        collect_acc_sequence_number(grpc_url.to_string(), sender_account_id.clone())
            .await
            .expect("Unable to fetch account and sequence number");

    let rpc_client = rpc::client::HttpClient::new(rpc_url).unwrap();

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
        account_number,
    )
    .unwrap();
    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();
    let tx_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();
    println!("tx_response:{:?}", tx_response);
}

async fn collect_acc_sequence_number(
    grpc_url: String,
    account_id: AccountId,
) -> Result<(u64, u64), CosmWasmConnectorError> {
    let grpc_url = grpc_url.clone();
    let end_point = tonic::transport::Endpoint::new(grpc_url).unwrap();
    let mut grpc_client = QueryClient::connect(end_point).await.unwrap();
    let query = QueryAccountRequest {
        address: account_id.to_string(),
    };

    match grpc_client.account(query).await {
        Ok(resp) => {
            let resp_account = resp.into_inner().account.unwrap_or(Default::default());
            let account = BaseAccount::decode(resp_account.value.as_slice()).unwrap();
            Ok((account.account_number, account.sequence))
        }
        Err(_) => Err(CosmWasmConnectorError::InvalidGRPCResponse),
    }
}
