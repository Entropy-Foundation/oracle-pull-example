use aptos_pull_client::aptos_connector::{invoke_aptos_chain, AptosConfig, AptosConnector};
use aptos_pull_client::pull_service::pull_response::Resp;
use aptos_pull_client::{pull_service, Client};

#[tokio::main]
async fn main() {
    let address = "<GRPC SERVER ADDRESS>".to_string(); // Set the gRPC server address
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
        let aptos_connector = AptosConnector::new(AptosConfig::new(
            "<--secret-key-->",
            "<--rpc-url-->",
            "<-contract-address-->",
            50000,
        ))
        .await
        .unwrap();
        invoke_aptos_chain(aptos_payload, aptos_connector).await
    }
}
