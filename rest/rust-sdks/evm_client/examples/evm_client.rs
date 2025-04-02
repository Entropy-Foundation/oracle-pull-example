use evm_pull_client::ethereum_connector::invoke_eth_chain;
use evm_pull_client::types::PullRequest;
use evm_pull_client::Client;

#[tokio::main]
async fn main() {
    let address = "<REST API SERVER ADDRESS>".to_string(); // Set the rest server address
    let client = Client::new(address).await.unwrap();

    // Create a PullRequest
    let request = PullRequest {
        pair_indexes: vec![0, 21],     // Set the pair indexes as an array
        chain_type: "evm".to_string(), // Set the chain type (evm, sui, aptos)
    };

    // Call the get_proof function and handle the result
    match client.get_proof(&request).await {
        Ok(response) => {
            invoke_eth_chain(response).await;
        }
        Err(status) => {
            eprint!("{:?}", status);
        }
    }
}
