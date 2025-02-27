use std::time::Instant;
use std::{error::Error, str::FromStr};

use crate::{
    constants::AORI_HTTP_PRODUCTION_API, OrderRecord, QuoteRequest, QuoteResponse, SignerType,
    SwapRequest, SwapResponse,
};
use ethers::signers::Signer;
// use crate::types::{SwapRequest, SwapResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChainInfo {
    pub chain_key: String,
    pub chain_id: u32,
    pub eid: i32,
    pub address: String,
    pub blocktime: String,
}

pub struct PollOrderStatusOptions {
    pub on_status_change: Option<Box<dyn Fn(String, OrderRecord) + Send + Sync>>,
    pub on_complete: Option<Box<dyn Fn(OrderRecord) + Send + Sync>>,
    pub on_error: Option<Box<dyn Fn(&Box<dyn Error>) + Send + Sync>>,
    pub interval: Option<u64>,
    pub timeout: Option<u64>,
}
pub async fn poll_order_status(
    order_hash: String,
    base_url: Option<String>,
    options: PollOrderStatusOptions,
) -> Result<OrderRecord, Box<dyn Error>> {
    let url = base_url.map_or(AORI_HTTP_PRODUCTION_API.to_string(), |url| url);
    let interval = std::time::Duration::from_millis(options.interval.map_or(1000, |interval| interval));
    let timeout_duration = std::time::Duration::from_millis(options.timeout.map_or(60000, |timeout| timeout));

    let mut last_status = String::new();
    let start_time = Instant::now();

    // Polling loop
    loop {
        // Check timeout
        if start_time.elapsed() > timeout_duration {
            let error = Box::new(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Order status polling timed out",
            )) as Box<dyn Error>;
            if let Some(on_error) = &options.on_error {
                on_error(&error);
            }
            return Err(error);
        }

        // Fetch order status
        let response = match reqwest::get(format!("{}/order/{}", url, order_hash)).await {
            Ok(resp) => resp,
            Err(err) => {
                let error = Box::new(err) as Box<dyn Error>;
                if let Some(on_error) = &options.on_error {
                    on_error(&error);
                }
                return Err(error);
            }
        };

        // Check response status
        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            let error = Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to fetch order status: {}", text),
            )) as Box<dyn Error>;

            if let Some(on_error) = &options.on_error {
                on_error(&error);
            }
            return Err(error);
        }

        // Parse the order
        let order: OrderRecord = match response.json::<OrderRecord>().await {
            Ok(order) => order,
            Err(err) => {
                let error = Box::new(err) as Box<dyn Error>;
                if let Some(on_error) = &options.on_error {
                    on_error(&error);
                }
                return Err(error);
            }
        };

        // Notify if status has changed
        if order.status != last_status {
            last_status = order.status.clone();
            if let Some(on_status_change) = &options.on_status_change {
                on_status_change(order.status.clone(), order.clone());
            }
        }

        // Check if order is complete
        if order.status == "filled" && order.dst_tx.is_some() {
            if let Some(on_complete) = &options.on_complete {
                on_complete(order.clone());
            }
            return Ok(order);
        }

        // Wait before next poll
        tokio::time::sleep(interval).await;
    }
}

pub async fn get_chains(base_url: String) -> Result<Vec<ChainInfo>, reqwest::Error> {
    let url = format!("{}/chains", base_url);

    let response = reqwest::get(url).await?;
    let chains: Vec<ChainInfo> = response.json::<Vec<ChainInfo>>().await?;

    Ok(chains)
}

pub async fn get_quote(
    request: QuoteRequest,
    base_url: Option<String>,
) -> Result<QuoteResponse, reqwest::Error> {
    let url = base_url.map_or(AORI_HTTP_PRODUCTION_API.to_string(), |url| url);

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/quote", url))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    response.json::<QuoteResponse>().await
}

pub async fn submit_swap(
    request: SwapRequest,
    base_url: Option<String>,
) -> Result<SwapResponse, reqwest::Error> {
    let url = base_url.map_or(AORI_HTTP_PRODUCTION_API.to_string(), |url| url);

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/swap", url))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    response.json::<SwapResponse>().await
}

pub async fn sign_order(
    quote_response: QuoteResponse,
    signer: SignerType,
) -> Result<String, Box<dyn std::error::Error>> {
    let wallet = ethers::signers::LocalWallet::from_str(&signer.private_key)?;

    let signing_hash_hex = if quote_response.signing_hash.starts_with("0x") {
        &quote_response.signing_hash[2..]
    } else {
        &quote_response.signing_hash
    };

    let message_bytes = ethers::utils::hex::decode(signing_hash_hex)?;
    let signature = wallet.sign_message(&message_bytes).await?;

    Ok(signature.to_string())
}

// pub struct SwapClient {
//     client: Client,
//     base_url: String,
// }

// impl SwapClient {
//     pub fn new() -> Self {
//         Self {
//             client: Client::new(),
//             base_url: AORI_HTTP_API_URL.to_string(),
//         }
//     }

//     pub fn with_url(base_url: String) -> Self {
//         Self {
//             client: Client::new(),
//             base_url,
//         }
//     }

//     /// Get a quote for a swap
//     pub async fn get_quote(&self, request: SwapGetRequest) -> Result<SwapGetResponse, reqwest::Error> {
//         let url = format!("{}/swap", self.base_url);

//         self.client
//             .get(url)
//             .json(&request)
//             .send()
//             .await?
//             .json::<SwapGetResponse>()
//             .await
//     }

//     /// Submit a signed swap for execution
//     pub async fn submit_swap(&self, request: SwapGetResponse) -> Result<SwapGetResponse, reqwest::Error> {
//         let url = format!("{}/swap", self.base_url);

//         self.client
//             .post(url)
//             .json(&request)
//             .send()
//             .await?
//             .json::<SwapGetResponse>()
//             .await
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_get_quote() {
//         let client = SwapClient::new();

//         let request = SwapGetRequest {
//             input_token: "0x123...".to_string(),
//             input_amount: "1000000000000000000".to_string(),
//             input_chain_id: 1,
//             output_token: "0x456...".to_string(),
//             output_chain_id: Some(1),
//         };

//         let response = client.get_quote(request).await;
//         assert!(response.is_ok());
//     }
// }
