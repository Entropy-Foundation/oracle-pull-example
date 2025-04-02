use sui_pull_client::sui_connector::{invoke_sui_chain, SuiConfig, SuiConnector};
use sui_pull_client::types::{PullRequest, PullResponseSui};
use sui_pull_client::Client;

#[tokio::main]
async fn main() {
    let address = "<REST SERVER ADDRESS>".to_string(); // Set the rest server address
    let client = Client::new(address).await.unwrap();

    // Create a PullRequest
    let request = PullRequest {
        pair_indexes: vec![0, 21],     // Set the pair indexes as an array
        chain_type: "sui".to_string(), // Set the chain type (evm, sui, aptos, radix)
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

async fn call_contract(input: PullResponseSui) {
    let sui_connector = SuiConnector::new(SuiConfig::new(
        "<--secret-key-->",
        "<--rpc-url-->",
        "<-contract-address-->",
        300000000,
    ))
    .await
    .unwrap();
    invoke_sui_chain(input, sui_connector).await
}
