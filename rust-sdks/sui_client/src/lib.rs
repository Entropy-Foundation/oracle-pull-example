use crate::pull_service::pull_service_client::PullServiceClient;
use pull_service::{PullRequest, PullResponse};

pub mod errors;
pub mod sui_connector;
pub mod pull_service {
    tonic::include_proto!("pull_service");
}

pub struct Client {
    client: PullServiceClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn new(address: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = PullServiceClient::connect(address).await?;
        Ok(Self { client })
    }

    pub async fn get_proof(
        &mut self,
        request: &PullRequest,
    ) -> Result<PullResponse, Box<dyn std::error::Error>> {
        match self
            .client
            .get_proof(tonic::Request::new(request.clone()))
            .await
        {
            Ok(response) => Ok(response.into_inner()),
            Err(status) => Err(Box::new(status)),
        }
    }
}
