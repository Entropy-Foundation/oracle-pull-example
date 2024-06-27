use cosmwasm_pull_client::cosmwasm_connector::invoke_cosmwasm_chain;
use cosmwasm_pull_client::pull_service::pull_response::Resp;
use cosmwasm_pull_client::{pull_service, Client};

#[tokio::main]
async fn main() {
    let address = "<GRPC_SERVER_ADDRESS>".to_string(); // Set the gRPC server address
    let mut client = Client::new(address).await.unwrap();

    // Create a PullRequest
    let request = pull_service::PullRequest {
        pair_indexes: vec![0, 21, 61, 49], // Set the pair indexes as an array
        chain_type: "cosmwasm".to_string(),
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
    if let Some(Resp::Cosmwasm(cosmwasm_payload)) = input.resp {
        invoke_cosmwasm_chain(cosmwasm_payload).await
    }
}
