use cosmwasm_pull_client::cosmwasm_connector::invoke_cosmwasm_chain;
use cosmwasm_pull_client::types::PullRequest;
use cosmwasm_pull_client::Client;

#[tokio::main]
async fn main() {
    let address = "<REST API SERVER ADDRESS>".to_string(); // Set the rest server address
    let client = Client::new(address).await.unwrap();

    // Create a PullRequest
    let request = PullRequest {
        pair_indexes: vec![0, 21, 61, 49], // Set the pair indexes as an array
        chain_type: "cosmwasm".to_string(),
    };

    // Call the get_proof function and handle the result
    match client.get_proof(&request).await {
        Ok(response) => {
            invoke_cosmwasm_chain(response).await;
        }
        Err(status) => {
            eprint!("{:?}", status);
        }
    }
}
