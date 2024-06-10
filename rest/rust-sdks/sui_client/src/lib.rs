pub mod errors;
pub mod sui_connector;
pub mod types;

use crate::types::PullResponseSui;
use reqwest::Client as HttpClient;
use std::error::Error;
use types::PullRequest;

pub struct Client {
    client: HttpClient,
    base_url: String,
}

impl Client {
    pub async fn new(base_url: String) -> Result<Self, Box<dyn Error>> {
        let client = HttpClient::new();
        Ok(Self { client, base_url })
    }

    pub async fn get_proof(
        &self,
        request: &PullRequest,
    ) -> Result<PullResponseSui, Box<dyn Error>> {
        let url = format!("{}/get_proof", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(request)
            .send()
            .await?
            .error_for_status()?
            .json::<PullResponseSui>()
            .await?;
        Ok(response)
    }
}
