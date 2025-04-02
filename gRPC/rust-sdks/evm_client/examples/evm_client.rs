use evm_pull_client::ethereum_connector::invoke_eth_chain;
use evm_pull_client::pull_service::pull_response::Resp;
use evm_pull_client::{pull_service, Client};

#[tokio::main]
async fn main() {
    let address = "<GRPC SERVER ADDRESS>".to_string(); // Set the gRPC server address
    let mut client = Client::new(address).await.unwrap();

    // Create a PullRequest
    let request = pull_service::PullRequest {
        pair_indexes: vec![0, 21, 61, 49], // Set the pair indexes as an array
        chain_type: "evm".to_string(),     // Set the chain type (evm, sui, aptos)
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
    if let Some(Resp::Evm(evm)) = input.resp {
        invoke_eth_chain(evm).await
    }
}
