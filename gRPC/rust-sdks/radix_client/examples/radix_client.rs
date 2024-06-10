use radix_pull_client::radix_connector::invoke_radix_chain;
use radix_pull_client::pull_service::pull_response::Resp;
use radix_pull_client::{pull_service, Client};

#[tokio::main]
async fn main() {
    let address = "<GRPC_SERVER>".to_string(); // Set the gRPC server address
    let mut client = Client::new(address).await.unwrap();

    // Create a PullRequest
    let request = pull_service::PullRequest {
        pair_indexes: vec![0, 21, 61, 49], // Set the pair indexes as an array
        chain_type: "radix".to_string(),     // Set the chain type (evm, sui, aptos, radix)
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
async fn call_contract(input: pull_service::PullResponse) {
    if let Some(Resp::Radix(radix)) = input.resp {
        invoke_radix_chain(radix).await
    }
}
