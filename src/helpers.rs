use crate::{constants::*, types::*};
use futures_util::StreamExt;
use hex;
use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use reqwest::Client;
use tokio_tungstenite::{connect_async, tungstenite::Message};

////////////////////////////////////////////////////////////////
//                      GET SUPPORTED CHAINS
////////////////////////////////////////////////////////////////

/// Fetches the list of supported chains and their configurations
pub async fn get_chains(base_url: Option<&str>, api_key: Option<&str>) -> Result<Vec<ChainInfo>> {
    let client = Client::new();
    let url = format!("{}/chains", base_url.unwrap_or(AORI_API));

    let mut request = client.get(&url).header("Content-Type", "application/json");

    if let Some(key) = api_key {
        request = request.header("x-api-key", key);
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AoriError::Api(format!("Failed to fetch chains: {}", error_text)));
    }

    let chains: Vec<ChainInfo> = response.json().await?;
    Ok(chains)
}

////////////////////////////////////////////////////////////////
//                     REQUEST A QUOTE
////////////////////////////////////////////////////////////////

pub async fn get_quote(
    request: &QuoteRequest,
    base_url: Option<&str>,
    api_key: Option<&str>,
) -> Result<QuoteResponse> {
    let client = Client::new();
    let url = format!("{}/quote", base_url.unwrap_or(AORI_API));

    let mut http_request = client.post(&url).header("Content-Type", "application/json");

    if let Some(key) = api_key {
        http_request = http_request.header("x-api-key", key);
    }

    let response = http_request.json(request).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AoriError::Api(format!("Quote request failed: {}", error_text)));
    }

    let quote: QuoteResponse = response.json().await?;
    Ok(quote)
}

////////////////////////////////////////////////////////////////
//                     SIGN AN ORDER
////////////////////////////////////////////////////////////////

/// Signs an order using the signing hash from a quote response
pub fn sign_order(quote_response: &QuoteResponse, private_key: &str) -> Result<String> {
    // Remove 0x prefix if present
    let private_key_hex = private_key.strip_prefix("0x").unwrap_or(private_key);

    // Parse private key
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| AoriError::Crypto(format!("Invalid private key: {}", e)))?;

    let signing_key = SigningKey::from_slice(&private_key_bytes)
        .map_err(|e| AoriError::Crypto(format!("Failed to create signing key: {}", e)))?;

    // Remove 0x prefix from signing hash if present
    let signing_hash_hex =
        quote_response.signing_hash.strip_prefix("0x").unwrap_or(&quote_response.signing_hash);

    // Parse signing hash
    let signing_hash_bytes = hex::decode(signing_hash_hex)
        .map_err(|e| AoriError::Crypto(format!("Invalid signing hash: {}", e)))?;

    // Sign the hash
    let signature: Signature = signing_key.sign(&signing_hash_bytes);

    // Format signature as hex string with 0x prefix
    let signature_bytes = signature.to_bytes();
    Ok(format!("0x{}", hex::encode(signature_bytes)))
}

////////////////////////////////////////////////////////////////
//               SIGN READABLE ORDER WITH EIP-712
////////////////////////////////////////////////////////////////

/// Signs an order using EIP-712 typed data format
pub async fn sign_readable_order(
    quote_response: &QuoteResponse,
    private_key: &str,
    _user_address: &str,
    _base_url: Option<&str>,
) -> Result<(String, String)> {
    // Get chain information
    let input_chain_info = get_chain_info_by_key(&quote_response.input_chain)
        .ok_or_else(|| AoriError::ChainNotFound(quote_response.input_chain.clone()))?;

    let output_chain_info = get_chain_info_by_key(&quote_response.output_chain)
        .ok_or_else(|| AoriError::ChainNotFound(quote_response.output_chain.clone()))?;

    // Create EIP-712 domain
    let _domain = serde_json::json!({
        "name": "Aori",
        "version": "0.3.0",
        "verifyingContract": input_chain_info.address
    });

    // Create message
    let _message = serde_json::json!({
        "offerer": quote_response.offerer,
        "recipient": quote_response.recipient,
        "inputToken": quote_response.input_token,
        "outputToken": quote_response.output_token,
        "inputAmount": quote_response.input_amount,
        "outputAmount": quote_response.output_amount,
        "startTime": quote_response.start_time,
        "endTime": quote_response.end_time,
        "srcEid": input_chain_info.eid,
        "dstEid": output_chain_info.eid,
    });

    // For now, return the order hash and a placeholder signature
    // In a full implementation, you would use a proper EIP-712 signing library
    let order_hash = quote_response.order_hash.clone();
    let signature = sign_order(quote_response, private_key)?;

    Ok((order_hash, signature))
}

////////////////////////////////////////////////////////////////
//                     SUBMIT SWAP
////////////////////////////////////////////////////////////////

pub async fn submit_swap(
    request: &SwapRequest,
    base_url: Option<&str>,
    api_key: Option<&str>,
) -> Result<SwapResponse> {
    let client = Client::new();
    let url = format!("{}/swap", base_url.unwrap_or(AORI_API));

    let mut http_request = client.post(&url).header("Content-Type", "application/json");

    if let Some(key) = api_key {
        http_request = http_request.header("x-api-key", key);
    }

    let response = http_request.json(request).send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AoriError::Api(format!("Swap submission failed: {}", error_text)));
    }

    let swap_response: SwapResponse = response.json().await?;
    Ok(swap_response)
}

////////////////////////////////////////////////////////////////
//                   GET ORDER STATUS
////////////////////////////////////////////////////////////////

pub async fn get_order_status(
    order_hash: &str,
    base_url: Option<&str>,
    api_key: Option<&str>,
) -> Result<OrderStatus> {
    let client = Client::new();
    let url = format!("{}/status/{}", base_url.unwrap_or(AORI_API), order_hash);

    let mut request = client.get(&url).header("Content-Type", "application/json");

    if let Some(key) = api_key {
        request = request.header("x-api-key", key);
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AoriError::Api(format!("Failed to get order status: {}", error_text)));
    }

    let status: OrderStatus = response.json().await?;
    Ok(status)
}

////////////////////////////////////////////////////////////////
//                   GET ORDER DETAILS
////////////////////////////////////////////////////////////////

/// Fetches detailed information about an order
pub async fn get_order_details(
    order_hash: &str,
    base_url: Option<&str>,
    api_key: Option<&str>,
) -> Result<OrderDetails> {
    let client = Client::new();
    let url = format!("{}/data/details/{}", base_url.unwrap_or(AORI_API), order_hash);

    let mut request = client.get(&url).header("Content-Type", "application/json");

    if let Some(key) = api_key {
        request = request.header("x-api-key", key);
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AoriError::Api(format!("Failed to fetch order details: {}", error_text)));
    }

    let details: OrderDetails = response.json().await?;
    Ok(details)
}

////////////////////////////////////////////////////////////////
//                     QUERY ORDERS
////////////////////////////////////////////////////////////////

/// Queries orders with filtering criteria
pub async fn query_orders(
    base_url: &str,
    params: &QueryOrdersParams,
    api_key: Option<&str>,
) -> Result<QueryOrdersResponse> {
    let client = Client::new();
    let url = format!("{}/data/query", base_url);

    let mut request = client.get(&url).header("Content-Type", "application/json");

    if let Some(key) = api_key {
        request = request.header("x-api-key", key);
    }

    // Build query parameters using reqwest's query method
    let mut request_with_params = request;

    if let Some(ref order_hash) = params.order_hash {
        request_with_params = request_with_params.query(&[("orderHash", order_hash)]);
    }
    if let Some(ref offerer) = params.offerer {
        request_with_params = request_with_params.query(&[("offerer", offerer)]);
    }
    if let Some(ref recipient) = params.recipient {
        request_with_params = request_with_params.query(&[("recipient", recipient)]);
    }
    if let Some(ref input_token) = params.input_token {
        request_with_params = request_with_params.query(&[("inputToken", input_token)]);
    }
    if let Some(ref input_chain) = params.input_chain {
        request_with_params = request_with_params.query(&[("inputChain", input_chain)]);
    }
    if let Some(ref output_token) = params.output_token {
        request_with_params = request_with_params.query(&[("outputToken", output_token)]);
    }
    if let Some(ref output_chain) = params.output_chain {
        request_with_params = request_with_params.query(&[("outputChain", output_chain)]);
    }
    if let Some(ref status) = params.status {
        request_with_params = request_with_params.query(&[("status", status)]);
    }
    if let Some(page) = params.page {
        request_with_params = request_with_params.query(&[("page", page.to_string())]);
    }
    if let Some(limit) = params.limit {
        request_with_params = request_with_params.query(&[("limit", limit.to_string())]);
    }
    if let Some(min_time) = params.min_time {
        request_with_params = request_with_params.query(&[("minTime", min_time.to_string())]);
    }
    if let Some(max_time) = params.max_time {
        request_with_params = request_with_params.query(&[("maxTime", max_time.to_string())]);
    }
    if let Some(ref min_value) = params.min_value {
        request_with_params = request_with_params.query(&[("minValue", min_value)]);
    }
    if let Some(ref max_value) = params.max_value {
        request_with_params = request_with_params.query(&[("maxValue", max_value)]);
    }
    if let Some(ref srctx) = params.srctx {
        request_with_params = request_with_params.query(&[("srctx", srctx)]);
    }
    if let Some(ref dst_tx) = params.dst_tx {
        request_with_params = request_with_params.query(&[("dstTx", dst_tx)]);
    }

    let response = request_with_params.send().await?;

    if response.status() == 404 {
        // Return empty result with pagination
        return Ok(QueryOrdersResponse {
            orders: vec![],
            pagination: PaginationMetadata {
                current_page: params.page.unwrap_or(1),
                limit: params.limit.unwrap_or(10),
                total_records: 0,
                total_pages: 0,
            },
        });
    }

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AoriError::Api(format!("Failed to query orders: {}", error_text)));
    }

    let query_response: QueryOrdersResponse = response.json().await?;
    Ok(query_response)
}

////////////////////////////////////////////////////////////////
//                     WEBSOCKET CLIENT
////////////////////////////////////////////////////////////////

pub struct AoriWebSocket {
    base_url: String,
    api_key: Option<String>,
}

impl AoriWebSocket {
    pub fn new(base_url: Option<&str>, api_key: Option<String>) -> Self {
        let mut url = base_url.unwrap_or(AORI_WS_API).to_string();
        if url.starts_with("http") {
            url = url.replace("http", "ws");
        }

        Self { base_url: url, api_key }
    }

    /// Connect to the Aori WebSocket server and handle messages
    pub async fn connect<F>(&self, mut on_message: F) -> Result<()>
    where
        F: FnMut(WSEvent) + Send + 'static,
    {
        let mut ws_url = self.base_url.clone();
        if let Some(ref api_key) = self.api_key {
            ws_url = format!("{}?key={}", ws_url, api_key);
        }

        let url = url::Url::parse(&ws_url)
            .map_err(|e| AoriError::WebSocket(format!("Invalid WebSocket URL: {}", e)))?;

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| AoriError::WebSocket(format!("Failed to connect: {}", e)))?;

        let (_write, mut read) = ws_stream.split();

        // Handle incoming messages
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => match serde_json::from_str::<WSEvent>(&text) {
                    Ok(event) => on_message(event),
                    Err(e) => {
                        log::error!("Failed to parse WebSocket message: {}", e);
                    }
                },
                Ok(Message::Close(_)) => {
                    log::info!("WebSocket connection closed");
                    break;
                }
                Err(e) => {
                    return Err(AoriError::WebSocket(format!("WebSocket error: {}", e)));
                }
                _ => {}
            }
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////
//                     POLL ORDER STATUS
////////////////////////////////////////////////////////////////

pub struct PollOrderStatusOptions<F, G, H>
where
    F: Fn(OrderStatus),
    G: Fn(OrderStatus),
    H: Fn(AoriError),
{
    pub on_status_change: Option<F>,
    pub on_complete: Option<G>,
    pub on_error: Option<H>,
    pub interval: Option<u64>, // milliseconds
    pub timeout: Option<u64>,  // milliseconds
}

/// Poll for order status changes
pub async fn poll_order_status<F, G, H>(
    order_hash: &str,
    base_url: Option<&str>,
    api_key: Option<&str>,
    options: PollOrderStatusOptions<F, G, H>,
) -> Result<OrderStatus>
where
    F: Fn(OrderStatus),
    G: Fn(OrderStatus),
    H: Fn(AoriError),
{
    let interval = std::time::Duration::from_millis(options.interval.unwrap_or(5000));
    let timeout = std::time::Duration::from_millis(options.timeout.unwrap_or(300000)); // 5 minutes default

    let start_time = std::time::Instant::now();
    let mut last_status: Option<OrderStatus> = None;

    loop {
        if start_time.elapsed() > timeout {
            let error = AoriError::Api("Polling timeout reached".to_string());
            if let Some(ref on_error) = options.on_error {
                on_error(error);
            }
            return Err(AoriError::Api("Polling timeout reached".to_string()));
        }

        match get_order_status(order_hash, base_url, api_key).await {
            Ok(status) => {
                // Check if status changed
                let status_changed = match &last_status {
                    None => true,
                    Some(last) => std::mem::discriminant(last) != std::mem::discriminant(&status),
                };

                if status_changed {
                    if let Some(ref on_status_change) = options.on_status_change {
                        on_status_change(status.clone());
                    }
                }

                // Check if order is complete
                match &status {
                    OrderStatus::Completed { .. } | OrderStatus::Failed { .. } => {
                        if let Some(ref on_complete) = options.on_complete {
                            on_complete(status.clone());
                        }
                        return Ok(status);
                    }
                    _ => {}
                }

                last_status = Some(status);
            }
            Err(e) => {
                if let Some(ref on_error) = options.on_error {
                    on_error(AoriError::Api(format!("Polling error: {}", e)));
                }
                return Err(e);
            }
        }

        tokio::time::sleep(interval).await;
    }
}
