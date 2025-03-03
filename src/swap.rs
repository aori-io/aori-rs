use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{error::Error, str::FromStr};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::types::OrderRecord;
use crate::{
    constants::AORI_HTTP_PRODUCTION_API, QuoteRequest, QuoteResponse, SignerType, SwapRequest,
    SwapResponse,
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

pub struct WebSocketCloseEvent {
    pub code: u16,
    pub reason: String,
    pub was_clean: bool,
}

pub struct WebSocketError {
    pub message: String,
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
    let interval =
        std::time::Duration::from_millis(options.interval.map_or(1000, |interval| interval));
    let timeout_duration =
        std::time::Duration::from_millis(options.timeout.map_or(60000, |timeout| timeout));

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

    // Convert to H256
    let hash_bytes = ethers::utils::hex::decode(signing_hash_hex)?;
    let mut hash_array = [0u8; 32];
    hash_array.copy_from_slice(&hash_bytes);
    let hash = ethers::types::H256::from(hash_array);

    // Sign the raw hash without EIP-191 prefix
    let signature = wallet.sign_hash(hash)?;

    Ok(signature.to_string())
}

pub struct AoriWebSocketClient {
    ws: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    options: AoriWebSocket,
    base_url: String,
}

impl AoriWebSocketClient {
    pub fn new(base_url: Option<String>, options: AoriWebSocket) -> Self {
        let url = base_url
            .unwrap_or_else(|| AORI_HTTP_PRODUCTION_API.to_string())
            .replace("https", "wss");

        Self { ws: Arc::new(Mutex::new(None)), options, base_url: url }
    }

    /// Connect to the Aori WebSocket server
    pub async fn connect(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let ws_url = Url::parse(&format!("{}/ws", self.base_url))?;

        println!("Connecting to WebSocket at {}", ws_url);

        // Use tungstenite's IntoClientRequest to handle protocol headers properly
        let mut request = ws_url.into_client_request()?;

        // Add custom headers
        request.headers_mut().insert("Origin", "https://client.aori.io".parse().unwrap());
        request.headers_mut().insert("User-Agent", "aori-rs-sdk/1.0".parse().unwrap());

        // Let connect_async handle the connection with proper headers
        let (mut ws_stream, response) = connect_async(request).await?;

        // Save the WebSocket stream
        *self.ws.lock().await = Some(ws_stream);

        // Call the onConnect callback
        if let Some(on_connect) = &self.options.on_connect {
            on_connect();
        }

        // Create a new barrier to ensure handler is ready
        let barrier = Arc::new(tokio::sync::Barrier::new(2));
        let barrier_clone = barrier.clone();

        // Start listening for messages in a separate task
        let ws_clone = Arc::clone(&self.ws);
        let options_clone = self.options.clone();

        tokio::spawn(async move {
            println!("Message handler preparing to start");
            // Signal we're ready to receive messages
            barrier_clone.wait().await;
            println!("Message handler now running");
            Self::handle_messages(ws_clone, options_clone).await;
        });

        // Wait for the handler to be ready
        barrier.wait().await;
        // Send initial subscription message
        println!("Sending subscription message");
        let sub_msg = r#"{"jsonrpc":"2.0","method":"subscribe","params":["orders"],"id":1}"#;
        self.send_message(sub_msg.to_string()).await?;

        // Add proper ping handling
        let ping_ws = Arc::clone(&self.ws);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(15));
            loop {
                interval.tick().await;
                let mut lock = ping_ws.lock().await;
                if let Some(stream) = lock.as_mut() {
                    if let Err(e) = stream.send(Message::Ping(vec![])).await {
                        println!("Ping failed: {}", e);
                        break;
                    }
                }
            }
        });

        println!("Connected and subscribed to WebSocket");
        Ok(())
    }

    async fn handle_messages(
        ws: Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
        options: AoriWebSocket,
    ) {
        loop {
            let message;
            {
                // Scope for lock
                let mut lock = ws.lock().await;
                let stream = match lock.as_mut() {
                    Some(s) => s,
                    None => break,
                };

                message = match stream.next().await {
                    Some(result) => {
                        println!("Result: {:?}", result);
                        result
                    }
                    None => break,
                };
            }

            println!(
                "Raw message received type: {:?}",
                message.as_ref().map(|m| match m {
                    Message::Text(_) => "Text",
                    Message::Binary(_) => "Binary",
                    Message::Ping(_) => "Ping",
                    Message::Pong(_) => "Pong",
                    Message::Close(_) => "Close",
                    Message::Frame(_) => "Frame",
                })
            );

            match message {
                Ok(Message::Text(text)) => {
                    println!("Received text: {}", text);

                    // First try parsing as a generic JSON Value to understand the structure
                    match serde_json::from_str::<serde_json::Value>(&text) {
                        Ok(value) => {
                            // Only try parsing as OrderRecord if it looks like one
                            if value.is_object() && value.get("status").is_some() {
                                if let Some(on_message) = &options.on_message {
                                    match serde_json::from_str::<OrderRecord>(&text) {
                                        Ok(order) => on_message(order),
                                        Err(e) => println!("Not an order record: {}", e),
                                    }
                                }
                            } else {
                                println!("Received non-order JSON: {}", value);
                            }
                        }
                        Err(e) => println!("Invalid JSON: {}", e),
                    }
                }
                Ok(Message::Ping(data)) => {
                    println!("Received ping, responding with pong");
                    let mut lock = ws.lock().await;
                    if let Some(stream) = lock.as_mut() {
                        if let Err(e) = stream.send(Message::Pong(data)).await {
                            println!("Failed to send pong: {}", e);
                        }
                    }
                }
                Ok(Message::Close(frame)) => {
                    println!("Server closed connection: {:?}", frame);
                    if let Some(on_disconnect) = &options.on_disconnect {
                        on_disconnect(WebSocketCloseEvent {
                            code: frame.as_ref().map_or(1000, |f| f.code.into()),
                            reason: frame.map_or("".to_string(), |f| f.reason.to_string()),
                            was_clean: true,
                        });
                    }
                    break;
                }
                Ok(_) => {} // Ignore other message types
                Err(e) => {
                    println!("WebSocket error: {}", e);
                    if let Some(on_error) = &options.on_error {
                        on_error(WebSocketError { message: e.to_string() });
                    }
                    break;
                }
            }
        }

        println!("Message handler loop terminated");
    }

    /// Disconnect from the WebSocket server
    pub async fn disconnect(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut ws_lock = self.ws.lock().await;
        if let Some(mut ws) = ws_lock.take() {
            ws.close(None).await?;
        }
        Ok(())
    }

    /// Check if the WebSocket is currently connected
    pub async fn is_connected(&self) -> bool {
        self.ws.lock().await.is_some()
    }

    /// Send a message to the WebSocket
    pub async fn send_message(&self, message: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut ws_lock = self.ws.lock().await;
        if let Some(ws) = ws_lock.as_mut() {
            ws.send(Message::Text(message)).await?;
            println!("Sending message:");
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "WebSocket is not connected",
            )))
        }
    }
}

// Make AoriWebSocket cloneable to share between threads
#[derive(Clone, Default)]
pub struct AoriWebSocket {
    pub on_message: Option<Arc<dyn Fn(OrderRecord) + Send + Sync>>,
    pub on_connect: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_disconnect: Option<Arc<dyn Fn(WebSocketCloseEvent) + Send + Sync>>,
    pub on_error: Option<Arc<dyn Fn(WebSocketError) + Send + Sync>>,
}

impl std::fmt::Debug for AoriWebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AoriWebSocket")
            .field("on_message", &self.on_message.as_ref().map(|_| "Fn(OrderRecord)"))
            .field("on_connect", &self.on_connect.as_ref().map(|_| "Fn()"))
            .field("on_disconnect", &self.on_disconnect.as_ref().map(|_| "Fn(WebSocketCloseEvent)"))
            .field("on_error", &self.on_error.as_ref().map(|_| "Fn(WebSocketError)"))
            .finish()
    }
}
