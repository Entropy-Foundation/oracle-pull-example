use crate::errors::ConnectorError;
use crate::pull_service::PullResponseAptos;
use aptos::common::utils::{chain_id, get_sequence_number};
use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::rest_client::Client;
use aptos_sdk::transaction_builder::TransactionFactory;
use aptos_sdk::types::LocalAccount;
use aptos_types::transaction::{EntryFunction, SignedTransaction, TransactionPayload};
use ed25519_dalek::{PublicKey, SecretKey};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{fmt::Display, future::Future};

use tiny_keccak::{Hasher, Sha3};

const MODULE: &str = "<CONTRACT MODULE>"; // Module name of your contract. Ex. pull_example
const ENTRY: &str = "<CONTRACT FUNCTION>"; // Module function name of your contract. Ex. get_pair_price

pub async fn invoke_aptos_chain(payload: PullResponseAptos, aptos_connector: AptosConnector) {
    let account = Account::from_secret_key(aptos_connector.secret_key).unwrap();

    let address = AccountAddress::from_hex_literal(&aptos_connector.sc_addr).unwrap();

    let aptos_arg = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(address, Identifier::new(MODULE).unwrap()),
        Identifier::new(ENTRY).unwrap(),
        vec![],
        vec![
            bcs::to_bytes(
                &AccountAddress::from_hex_literal(&payload.oracle_holder_object).unwrap(),
            )
            .unwrap(),
            bcs::to_bytes(&payload.proof_bytes).unwrap(),
        ],
    ));

    let sequence_number = aptos_connector
        .client
        .with_aptos(|aptos| account.get_seq_num(aptos))
        .await
        .unwrap();
    let tx = aptos_connector
        .client
        .with_aptos(|aptos| {
            account.setup_transaction(
                &aptos_arg,
                aptos,
                sequence_number,
                aptos_connector.gas_budget,
            )
        })
        .await
        .unwrap();

    let response = aptos_connector
        .client
        .with_aptos(|aptos| aptos.submit_and_wait(&tx))
        .await
        .unwrap()
        .into_inner();

    println!("{:?}", response.transaction_info().unwrap().hash);
}

pub struct AptosConfig<'a> {
    secret_key: &'a str,
    client_url: &'a str,
    sc_address: &'a str,
    gas_budget: u64,
}

impl<'a> AptosConfig<'a> {
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
pub struct AptosConnector {
    secret_key: String,
    client: ClientWrapper,
    sc_addr: String,
    gas_budget: u64,
}

impl AptosConnector {
    pub async fn new(conf: AptosConfig<'_>) -> Result<Self, ConnectorError> {
        let url = conf
            .client_url
            .parse::<reqwest::Url>()
            .map_err(|_| ConnectorError::InvalidUrl)?;
        let client = Client::new(url);

        Ok(Self {
            client: ClientWrapper::new(client, None),
            secret_key: conf.secret_key.to_string(),
            sc_addr: conf.sc_address.to_string(),
            gas_budget: conf.gas_budget,
        })
    }
}

#[derive(Clone)]
pub struct ClientWrapper {
    primary: Client,
    backup: Option<Client>,
    is_backup: Arc<AtomicBool>,
}

impl ClientWrapper {
    pub fn new(primary: Client, backup: Option<Client>) -> Self {
        Self {
            primary,
            backup,
            is_backup: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn with_aptos<'a, R, E: Display, F: Future<Output = Result<R, E>> + 'a>(
        &'a self,
        mut f: impl FnMut(&'a Client) -> F,
    ) -> Result<R, E> {
        let Some(backup) = self.backup.as_ref() else {
            log::debug!("aptos: sending request with primary connector");
            return f(&self.primary).await;
        };

        if self.is_backup.load(Ordering::SeqCst) {
            log::debug!("aptos: sending request with backup connector");
            return f(backup).await;
        }

        log::debug!("aptos: sending request with primary connector");
        let res = f(&self.primary).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                log::warn!("aptos client error: {e}, falling back to backup transport");
                self.is_backup.store(true, Ordering::SeqCst);
                f(backup).await
            }
        }
    }
}

pub struct Account {
    signing_key: SecretKey,
    sender_key: Ed25519PrivateKey,
}

impl Account {
    pub fn from_secret_key(input: String) -> Result<Self, ConnectorError> {
        let input = input.trim_start_matches("0x");
        let h = hex::decode(input).unwrap();
        let signing_key = SecretKey::from_bytes(&h).unwrap();
        let sender_key = Ed25519PrivateKey::try_from(&*signing_key.to_bytes().to_vec()).unwrap();
        Ok(Account {
            signing_key,
            sender_key,
        })
    }

    pub fn address(&self) -> String {
        self.auth_key()
    }

    pub fn auth_key(&self) -> String {
        let mut sha3 = Sha3::v256();
        sha3.update(PublicKey::from(&self.signing_key).as_bytes());
        sha3.update(&[0u8]);

        let mut output = [0u8; 32];
        sha3.finalize(&mut output);
        hex::encode(output)
    }

    pub fn to_address(&self) -> Result<AccountAddress, ConnectorError> {
        AccountAddress::from_hex_literal(&format!("0x{}", self.address())).map_err(|e| e.into())
    }

    pub async fn get_seq_num(&self, rest_client: &Client) -> Result<u64, ConnectorError> {
        get_sequence_number(rest_client, self.to_address()?)
            .await
            .map_err(|e| e.into())
    }

    pub async fn setup_transaction(
        &self,
        payload: &TransactionPayload,
        rest_client: &Client,
        sequence_number: u64,
        gas_budget: u64,
    ) -> Result<SignedTransaction, ConnectorError> {
        let transaction_factory = TransactionFactory::new(chain_id(rest_client).await.unwrap())
            .with_gas_unit_price(100)
            .with_max_gas_amount(gas_budget);

        let sender_key = self.sender_key.clone();
        let sender_account =
            &mut LocalAccount::new(self.to_address()?, sender_key, sequence_number);
        Ok(sender_account
            .sign_with_transaction_builder(transaction_factory.payload(payload.clone())))
    }
}
