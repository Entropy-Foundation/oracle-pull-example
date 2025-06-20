use supra_pull_client::supra_connector::{invoke_supra_chain, SupraConfig, SupraConnector};
use supra_pull_client::pull_service::pull_response::Resp;
use supra_pull_client::{pull_service, Client};

#[tokio::main]
async fn main() {
    env_logger::init();
    let address = "https://testnet-dora-2.supra.com:443".to_string(); // Set the gRPC server address
    let mut client = Client::new(address).await.unwrap();

    // Create a PullRequest
    let request = pull_service::PullRequest {
        pair_indexes: vec![0, 21, 61, 49], // Set the pair indexes as an array
        chain_type: "aptos".to_string(),   // Set the chain type (evm, sui, aptos, radix)
    };

    // Call the get_proof function and handle the result
    match client.get_proof(&request).await {
        Ok(response) => {
            call_contract(response).await;
        }
        Err(status) => {
            eprint!("{:?}", status);
        }
    }
}

async fn call_contract(input: pull_service::PullResponse) {
    if let Some(Resp::Aptos(aptos_payload)) = input.resp {
        let supra_connector = SupraConnector::new(SupraConfig::new(
            "<--secret-key-->",
            "<--rpc-url-->",
            "<-contract-address-->",
            50000,
        ))
        .await
        .unwrap();
        invoke_supra_chain(aptos_payload, supra_connector).await
    }
}
