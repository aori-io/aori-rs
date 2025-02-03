use reqwest::Client;
use crate::types::{SwapGetRequest, SwapGetResponse};
use crate::constants::AORI_HTTP_API_URL;

pub struct SwapClient {
    client: Client,
    base_url: String,
}

impl SwapClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: AORI_HTTP_API_URL.to_string(),
        }
    }

    pub fn with_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Get a quote for a swap
    pub async fn get_quote(&self, request: SwapGetRequest) -> Result<SwapGetResponse, reqwest::Error> {
        let url = format!("{}/swap", self.base_url);
        
        self.client
            .get(url)
            .json(&request)
            .send()
            .await?
            .json::<SwapGetResponse>()
            .await
    }

    /// Submit a signed swap for execution 
    pub async fn submit_swap(&self, request: SwapGetResponse) -> Result<SwapGetResponse, reqwest::Error> {
        let url = format!("{}/swap", self.base_url);

        self.client
            .post(url)
            .json(&request) 
            .send()
            .await?
            .json::<SwapGetResponse>()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_quote() {
        let client = SwapClient::new();
        
        let request = SwapGetRequest {
            input_token: "0x123...".to_string(),
            input_amount: "1000000000000000000".to_string(),
            input_chain_id: 1,
            output_token: "0x456...".to_string(),
            output_chain_id: Some(1),
        };

        let response = client.get_quote(request).await;
        assert!(response.is_ok());
    }
}
