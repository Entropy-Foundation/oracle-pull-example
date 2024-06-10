use crate::pull_contract::MockOracleClient;
use crate::types::PullResponseEvm;
use ethers::core::k256::FieldBytes;
use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::LocalWallet,
    types::Address,
    utils::secret_key_to_address,
};
use std::str::FromStr;
use std::sync::Arc;

pub async fn invoke_eth_chain(evm: PullResponseEvm) {
    let rpc_url = "<RPC URL>"; // Rpc url for desired chain
    let secret_key = "<PRIVATE KEY>"; // Your Private Key
    let contract_address = "<CONTRACT ADDRESS>"; //Address of your smart contract

    let http_provider = Provider::<Http>::try_from(rpc_url)
        .map_err(|_| eprint!("Invalid rpc url"))
        .unwrap();

    let network_chain_id = http_provider.get_chainid().await.unwrap().as_u64();

    let secret_key_bytes = hex::decode(secret_key)
        .map_err(|_| eprint!("Invalid rpc url"))
        .unwrap();

    let signer_key = SigningKey::from_bytes(FieldBytes::from_slice(secret_key_bytes.as_slice()))
        .map_err(|_| eprint!("Invalid secret key"))
        .unwrap();

    let addr = secret_key_to_address(&signer_key);

    let wallet = LocalWallet::new_with_signer(signer_key, addr, network_chain_id);

    let signed_prov = SignerMiddleware::new(http_provider, wallet);

    let provider = Arc::new(signed_prov);

    let sc_addr = Address::from_str(contract_address)
        .map_err(|_| eprint!("Invalid contract address"))
        .unwrap();

    let sc = MockOracleClient::new(sc_addr, provider.clone());
    let bytes = hex::decode(evm.proof_bytes).unwrap();
    let mut call = sc.verify_oracle_proof(bytes.into());
    call = call.legacy();
    let result = call.send().await.unwrap();
    println!("{:?}", result.await);
}
