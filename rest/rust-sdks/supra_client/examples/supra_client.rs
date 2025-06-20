use supra_pull_client::supra_connector::{invoke_supra_chain, SupraConfig, SupraConnector};
use supra_pull_client::types::{PullResponseSupra, PullSupraRequest};
use supra_pull_client::Client;

#[tokio::main]
async fn main() {
    env_logger::init();
    let address = "<REST API SERVER ADDRESS>".to_string(); // Set the rest server address
    let client = Client::new(address).await.unwrap();

    // Create a PullRequest
    let request = PullSupraRequest {
        pair_indexes: vec![0, 21],       // Set the pair indexes as an array
        chain_type: "aptos".to_string(), // Set the chain type (evm, sui, aptos, radix)
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

async fn call_contract(input: PullResponseSupra) {
    let supra_connector = SupraConnector::new(SupraConfig::new(
        "<--secret-key-->",
        "<--rpc-url-->",
        "<-contract-address-->",
        50000,
    ))
        .await
        .unwrap();
    invoke_supra_chain(input, supra_connector).await;
}
