use std::time::Duration;
use radix_transactions::prelude::*;
use reqwest::{header::*};
use reqwest::Client;
use crate::gateway::{GatewayStatus, TransactionStatus, TransactionSubmit};
use crate::pull_service::PullResponseRadix;
use scrypto::prelude::*;

pub type PairIndex = u32;
pub type PriceType = u128;
pub type TimestampType = u64;
pub type DecimalType = u16;
pub type Round = u64;

#[derive(ScryptoSbor, Debug, Clone, Eq, PartialEq)]
pub struct PriceData {
    pub pair_index: PairIndex,
    pub price: PriceType,
    pub timestamp: TimestampType,
    pub decimal: DecimalType,
    pub round: Round,
}
#[derive(ScryptoSbor, Debug, Clone, PartialEq, Eq)]
pub struct CommitteeFeedWithProof {
    pub committee_feed: PriceData,
    pub proof: Vec<[u8; 32]>,
}

#[derive(ScryptoSbor, Debug, Clone, Eq, PartialEq)]
pub struct PriceDetailsWithCommittee {
    pub committee_id: u64,
    pub root: Vec<u8>,
    pub sig: Bls12381G2Signature,
    pub committee_data: Vec<CommitteeFeedWithProof>,
}

#[derive(ScryptoSbor, Debug, Clone, Eq, PartialEq)]
pub struct OracleProof {
    pub data: Vec<PriceDetailsWithCommittee>,
}

const GATEWAY_URL : &str = "https://stokenet.radixdlt.com";
const NETWORK_ID : u8 = 2;
const LOGICAL_NAME: &str = "stokenet";
const HRP_SUFFIX: &str = "tdx_2_";


pub async fn invoke_radix_chain(radix_response: PullResponseRadix) {
    // let oracle_proof_bytes : OracleProof = scrypto_decode(&radix_response.proof_bytes).unwrap();
    // println!("oracle proof: {:#?}", oracle_proof_bytes);
    let oracle_proof_bytes = radix_response.proof_bytes;

    let network_definition = NetworkDefinition {
        id: NETWORK_ID,
        logical_name: LOGICAL_NAME.to_string(),
        hrp_suffix: HRP_SUFFIX.to_string(),
    };

    let hash_encoder = TransactionHashBech32Encoder::new(&network_definition);

    let address_decoder = AddressBech32Decoder::new(&network_definition);
    let address_encoder = AddressBech32Encoder::new(&network_definition);

    let component_address =
        ComponentAddress::try_from_bech32(&address_decoder, "<COMPONENT_ADDRESS>")
            .expect("Invalid component address");

    let client = Client::new();
    let mut epoch = get_epoch(&client).await;

    let mut index_set = IndexSet::new();
    index_set.insert(NonFungibleLocalId::from_str("{<NONFUNGIBLE-RUID>}").unwrap());

    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(
            DynamicGlobalAddress::from(component_address),
            "<COMPONENT METHOD>",
            manifest_args!(oracle_proof_bytes,index_set),
        )
        .build();

    let private_key = Ed25519PrivateKey::from_bytes(&hex::decode("<PRIVATE_KEY>").unwrap()).unwrap();
    let public_address = ComponentAddress::virtual_account_from_public_key(&private_key.public_key());

    let public_address_string = public_address.to_string(&address_encoder);

    println!("Account Address:{:?}", public_address_string);

    let transaction = TransactionBuilder::new()
        .header(TransactionHeaderV1 {
            network_id: NETWORK_ID,
            start_epoch_inclusive: Epoch::of(epoch),
            end_epoch_exclusive: Epoch::of(epoch+10),
            nonce: 3,
            notary_public_key: private_key.public_key().into(),
            notary_is_signatory: false,
            tip_percentage: 0,
        })
        .manifest(manifest)
        .notarize(&private_key)
        .build();

    let intent_hash = transaction.prepare().unwrap().intent_hash();
    let notarized_transaction_bytes = transaction.to_payload_bytes().unwrap();
    let notarized_transaction_hex = hex::encode(&notarized_transaction_bytes);

    let tx_resp = transaction_submit(&client, notarized_transaction_hex).await;

    let intent_hash_str = hash_encoder.encode(&intent_hash).unwrap();
    println!("Tx Hash: {:?}", intent_hash_str);
    println!("Tx Resp: {:?}", tx_resp);
    if !tx_resp.duplicate {
        loop {
            let tx_status = transaction_status(&client, &intent_hash_str).await;
            if tx_status.status != "Pending" {
                println!("Transaction Status:{:#?}", tx_status);
                break;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}

pub async fn get_epoch(client: &Client) -> u64 {
    let resp =
        client
        .post(format!("{}/status/gateway-status", GATEWAY_URL))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "oracle-pull-example")
        .send().await.expect("Unable to get status");
    
    let body : GatewayStatus = resp.json().await.expect("Unable to parse json on status");
    body.ledger_state.epoch
}

pub async fn transaction_status(client: &Client, intent_hash: &str) -> TransactionStatus {
    let mut map = HashMap::new();
        map.insert("intent_hash", intent_hash);

    let resp = client
        .post(format!("{}/transaction/status", GATEWAY_URL))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "oracle-pull-example")
        .json(&map)
        .send()
        .await
        .expect("Unable to fetch tx status");

    resp.json().await.expect("Unable to parse tx status")
}

pub async fn transaction_submit(client: &Client, tx_bytes_hex: String) -> TransactionSubmit {
    let mut map = HashMap::new();
    map.insert("notarized_transaction_hex", tx_bytes_hex);
    let resp =
          client.post(format!("{}/transaction/submit", GATEWAY_URL))
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "oracle-pull-example")
        .json(&map)
        .send().await.expect("Unable to send the transaction");

    resp.json().await.expect("Unable to parse tx submit response")
}