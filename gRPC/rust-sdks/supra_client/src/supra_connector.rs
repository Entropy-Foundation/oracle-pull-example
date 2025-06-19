use crate::errors::ConnectorError;
use crate::pull_service::PullResponseAptos;
use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::crypto::{PrivateKey, SigningKey};
use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::transaction_builder::TransactionFactory;
use aptos_types::chain_id::ChainId;
use aptos_types::transaction::{EntryFunction, SignedTransaction, TransactionPayload};
use ed25519_dalek::{PublicKey, SecretKey};
use reqwest::Url;
use sha3::Digest;
use std::time::Duration;
use tiny_keccak::{Hasher, Sha3};
use crate::types::{SupraAccountResponse, SupraTransaction};

const MODULE: &str = "<CONTRACT MODULE>"; // Module name of your contract. Ex. pull_example
const ENTRY: &str = "<CONTRACT FUNCTION>"; // Module function name of your contract. Ex. get_pair_price
const SUPRA_TX_PATH: &str = "/rpc/v1/transactions/submit";
const SUPRA_ACCOUNTS_PATH: &str = "rpc/v1/accounts/";
const SUPRA_CHAIN_ID_PATH: &str = "rpc/v1/transactions/chain_id";
const DEFAULT_TIMEOUT_FOR_REQUEST: Duration = Duration::from_secs(10);

pub async fn invoke_supra_chain(payload: PullResponseAptos, supra_connector: SupraConnector) {
    let account = Account::from_secret_key(supra_connector.secret_key).unwrap();

    let address = AccountAddress::from_hex_literal(&supra_connector.sc_addr).unwrap();

    let tx_args = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(address, Identifier::new(MODULE).unwrap()),
        Identifier::new(ENTRY).unwrap(),
        vec![],
        vec![bcs::to_bytes(&payload.proof_bytes).unwrap()],
    ));

    let sequence_number = get_sequence_for_supra(
        &supra_connector.network_base_path,
        account.to_address().unwrap().to_string(),
    )
        .await
        .unwrap();
    let chain_id: u8 = get_chain_id_for_supra(&supra_connector.network_base_path)
        .await
        .unwrap();

    let tx = account
        .setup_supra_transaction(
            &tx_args,
            sequence_number,
            chain_id,
            supra_connector.gas_budget,
        )
        .await
        .unwrap();

    let tx = SupraTransaction::Move(tx);
    match send_supra_tx(supra_connector.network_base_path, tx).await {
        Ok(tx_hash) => {
            log::info!("sent tx, data : {}", tx_hash);
        }
        Err(e) => log::error!("{e}"),
    }
}

pub struct SupraConfig<'a> {
    secret_key: &'a str,
    client_url: &'a str,
    sc_address: &'a str,
    gas_budget: u64,
}

impl<'a> SupraConfig<'a> {
    pub fn new(
        secret_key: &'a str,
        client_url: &'a str,
        sc_address: &'a str,
        gas_budget: u64,
    ) -> Self {
        Self {
            secret_key,
            client_url,
            sc_address,
            gas_budget,
        }
    }
}

#[derive(Clone)]
pub struct SupraConnector {
    secret_key: String,
    sc_addr: String,
    gas_budget: u64,
    network_base_path: Url,
}

impl SupraConnector {
    pub async fn new(conf: SupraConfig<'_>) -> Result<Self, ConnectorError> {
        let url = conf
            .client_url
            .parse::<reqwest::Url>()
            .map_err(|_| ConnectorError::InvalidUrl)?;

        Ok(Self {
            network_base_path: url,
            secret_key: conf.secret_key.to_string(),
            sc_addr: conf.sc_address.to_string(),
            gas_budget: conf.gas_budget,
        })
    }
}

#[derive(Clone)]
pub struct Account {
    auth_key: String,
    sender_key: Ed25519PrivateKey,
}

impl Account {
    /// Load from raw secret key
    pub fn from_secret_key(input: String) -> Result<Self, ConnectorError> {
        let input = input.trim_start_matches("0x");
        let h = hex::decode(input)?;
        let signing_key = SecretKey::from_bytes(&h)?;
        let sender_key = Ed25519PrivateKey::try_from(&*signing_key.to_bytes().to_vec())?;

        let auth_key = {
            let mut sha3 = Sha3::v256();
            sha3.update(PublicKey::from(&signing_key).as_bytes());
            sha3.update(&[0u8]);

            let mut output = [0u8; 32];
            sha3.finalize(&mut output);
            hex::encode(output)
        };
        Ok(Account {
            auth_key,
            sender_key,
        })
    }

    /// Get the account's address
    pub fn to_address(&self) -> Result<AccountAddress, ConnectorError> {
        AccountAddress::from_hex_literal(&format!("0x{}", self.auth_key)).map_err(|e| e.into())
    }

    /// Constructs a transaction from a payload and sign it
    pub async fn setup_supra_transaction(
        &self,
        payload: &TransactionPayload,
        sequence_number: u64,
        chain_id: u8,
        gas_limit: u64,
    ) -> Result<SignedTransaction, ConnectorError> {
        let transaction_factory = TransactionFactory::new(ChainId::new(chain_id))
            .with_max_gas_amount(gas_limit)
            .with_transaction_expiration_time(300);
        let raw_tx = transaction_factory
            .payload(payload.clone())
            .sender(self.to_address()?)
            .sequence_number(sequence_number)
            .build();
        let supra_hash_prefix = b"SUPRA::RawTransaction";
        let mut finalised_bytes = Self::create_sha3_hash(supra_hash_prefix);
        // Unwrap safety as RawTransaction already support Serialise trait
        let tx_bytes = &bcs::to_bytes(&raw_tx).unwrap();
        finalised_bytes.extend_from_slice(&tx_bytes);
        let signature = self
            .sender_key
            .sign_arbitrary_message(finalised_bytes.as_slice());
        let signed_tx = SignedTransaction::new(raw_tx, self.sender_key.public_key(), signature);
        Ok(signed_tx)
    }

    pub fn create_sha3_hash(bytes: &[u8]) -> Vec<u8> {
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(bytes);
        hasher.finalize().to_vec()
    }
}

async fn send_supra_tx(url: Url, tx: SupraTransaction) -> Result<String, ConnectorError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    let tx_url = url.join(SUPRA_TX_PATH)?;
    let res = client.post(tx_url).json(&tx).send().await?;
    let status = res.status();
    let response = res.text().await?;
    if status != 200 {
        return Err(ConnectorError::SupraTransaction(response));
    }
    Ok(response)
}
pub async fn get_sequence_for_supra(
    client_url: &Url,
    address: String,
) -> Result<u64, ConnectorError> {
    let client = reqwest::Client::builder()
        .timeout(DEFAULT_TIMEOUT_FOR_REQUEST)
        .build()?;
    let acc_url = client_url.join(&format!("{}{}", SUPRA_ACCOUNTS_PATH, address))?;
    let acc_json: SupraAccountResponse = client.get(acc_url).send().await?.json().await?;

    Ok(acc_json.sequence_number)
}

async fn get_chain_id_for_supra(client_url: &Url) -> Result<u8, ConnectorError> {
    let client = reqwest::Client::builder()
        .timeout(DEFAULT_TIMEOUT_FOR_REQUEST)
        .build()?;
    let chain_id_url = client_url.join(SUPRA_CHAIN_ID_PATH)?;
    let chain_id_bytes = client.get(chain_id_url).send().await?.text().await?;
    let chain_id: u8 = chain_id_bytes.parse()?;
    Ok(chain_id)
}
