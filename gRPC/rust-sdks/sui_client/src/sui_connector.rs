use crate::errors::ConnectorError;
use crate::pull_service::PullResponseSui;
use shared_crypto::intent::Intent;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fmt::Display, future::Future};
use sui_keys::keystore::{AccountKeystore, InMemKeystore};
use sui_sdk::json::SuiJsonValue;
use sui_sdk::rpc_types::{SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions};
use sui_sdk::{SuiClient, SuiClientBuilder};
use sui_types::base_types::ObjectID;
use sui_types::base_types::SuiAddress;
use sui_types::crypto::EncodeDecodeBase64;
use sui_types::crypto::SuiKeyPair;
use sui_types::transaction::{Transaction, TransactionData};

const MODULE: &str = "<CONTRACT MODULE>"; // Module name of your contract. Ex. pull_example
const ENTRY: &str = "<CONTRACT FUNCTION>"; // Module function name of your contract. Ex. get_pair_price
const CLOCK: &str = "0x6";

pub async fn invoke_sui_chain(payload: PullResponseSui, sui_connector: SuiConnector) {
    let sui_arg = vec![
        SuiJsonValue::from_str(&payload.dkg_object).unwrap(),
        SuiJsonValue::from_str(&payload.oracle_holder_object).unwrap(),
        SuiJsonValue::from_str(&payload.merkle_root_object).unwrap(),
        SuiJsonValue::from_str(CLOCK).unwrap(),
        SuiJsonValue::from_bcs_bytes(None, &payload.proof_bytes).unwrap(),
    ];
    let tx_data = sui_connector
        .client
        .with_sui(|sui_client| {
            sui_client.transaction_builder().move_call(
                sui_connector.get_sui_address().unwrap(),
                ObjectID::from_hex_literal(&sui_connector.sc_addr).unwrap(),
                MODULE,
                ENTRY,
                vec![],
                sui_arg.clone(),
                None,
                sui_connector.gas_budget,
                None,
            )
        })
        .await
        .unwrap();
    let transaction = sui_connector.sign_and_execute_tx(tx_data).await;
    if let Ok(transaction) = transaction {
        let transaction = transaction.digest.to_string();
        println!("{}", transaction);
    } else {
        println!("{:?}", transaction);
    }
}

pub struct SuiConfig<'a> {
    secret_key: &'a str,
    client_url: &'a str,
    sc_address: &'a str,
    gas_budget: u64,
}

impl<'a> SuiConfig<'a> {
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

pub struct SuiConnector {
    client: ClientWrapper,
    sc_addr: String,
    secret_key: String,
    gas_budget: u64,
}

#[derive(Clone)]
pub struct ClientWrapper {
    primary: SuiClient,
    backup: Option<SuiClient>,
    is_backup: Arc<AtomicBool>,
}

impl SuiConnector {
    pub async fn new(conf: SuiConfig<'_>) -> Result<Self, ConnectorError> {
        log::trace!("Start SuiConnector");
        let client = SuiClientBuilder::default()
            .build(conf.client_url)
            .await
            .map_err(|_| ConnectorError::InvalidUrl)?;

        let sui_connector = Self {
            client: ClientWrapper::new(client, None),
            sc_addr: conf.sc_address.to_string(),
            secret_key: conf.secret_key.to_string(),
            gas_budget: conf.gas_budget,
        };
        Ok(sui_connector)
    }

    pub fn get_sui_address(&self) -> Result<SuiAddress, ConnectorError> {
        let key_pair = SuiKeyPair::decode_base64(&self.secret_key)
            .map_err(|_| ConnectorError::InvalidSecretKey)?;
        let sui_address: SuiAddress = (&key_pair.public()).into();
        Ok(sui_address)
    }

    fn get_key_store(&self) -> Result<InMemKeystore, ConnectorError> {
        let key_pair = SuiKeyPair::decode_base64(&self.secret_key)
            .map_err(|_| ConnectorError::InvalidSecretKey)?;

        let mut key_store = InMemKeystore::default();
        key_store
            .add_key(None, key_pair)
            .map_err(|_| ConnectorError::InvalidSecretKey)?;
        Ok(key_store)
    }

    pub async fn sign_and_execute_tx(
        &self,
        tx_data: TransactionData,
    ) -> Result<SuiTransactionBlockResponse, ConnectorError> {
        let key_store = self.get_key_store()?;
        let owner = self.get_sui_address()?;
        let signature = key_store
            .sign_secure(&owner, &tx_data, Intent::sui_transaction())
            .map_err(|err| ConnectorError::SuiTransaction(err.to_string()))?;

        let tx = Transaction::from_data(tx_data, vec![signature]);
        let transaction = self
            .client
            .with_sui(|sui| {
                sui.quorum_driver_api().execute_transaction_block(
                    Transaction::from(tx.clone()),
                    SuiTransactionBlockResponseOptions::full_content(),
                    Some(sui_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForLocalExecution),
                )
            })
            .await.unwrap();
        Ok(transaction)
    }
}

impl ClientWrapper {
    pub fn new(primary: SuiClient, backup: Option<SuiClient>) -> Self {
        Self {
            primary,
            backup,
            is_backup: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn with_sui<'a, R, E: Display, F: Future<Output = Result<R, E>> + 'a>(
        &'a self,
        mut f: impl FnMut(&'a SuiClient) -> F,
    ) -> Result<R, E> {
        let Some(backup) = self.backup.as_ref() else {
            log::debug!("sui: sending request with primary connector");
            return f(&self.primary).await;
        };

        if self.is_backup.load(Ordering::SeqCst) {
            log::debug!("sui: sending request with backup connector");
            return f(backup).await;
        }

        log::debug!("sui: sending request with primary connector");
        let res = f(&self.primary).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                log::warn!("sui client error: {e}, falling back to backup transport");
                self.is_backup.store(true, Ordering::SeqCst);
                f(backup).await
            }
        }
    }
}
