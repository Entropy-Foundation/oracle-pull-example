use radix_pull_client::radix_connector::invoke_radix_chain;
use radix_pull_client::types::{PullRequest, PullResponseRadix};
use radix_pull_client::Client;

#[tokio::main]
async fn main() {
    let address = "<REST API SERVER ADDRESS>".to_string(); // Set the rest server address
    let client = Client::new(address).await.unwrap();

    // Create a PullRequest
    let request = PullRequest {
        pair_indexes: vec![0, 21],       // Set the pair indexes as an array
        chain_type: "radix".to_string(), // Set the chain type (evm, sui, aptos, radix)
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
//
async fn call_contract(input: PullResponseRadix) {
    invoke_radix_chain(input).await
}
