# Aori Rust SDK

![aori-rs banner](https://github.com/aori-io/.github/blob/main/assets/aori-rs.png)

[![https://devs.aori.io](https://img.shields.io/badge/🗨_telegram_chat-0088cc)](https://devs.aori.io) ![GitHub issues](https://img.shields.io/github/issues-raw/aori-io/aori-rs?color=blue)

## Getting Started

#### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
aori-rs = "0.3.0"
tokio = { version = "1.0", features = ["full"] }
```

#### Authorization

Interacting with the Aori API does not currently require an API key, although it is recommended you visit the [Aori Developer Portal](https://developers.aori.io) to receive an integrator ID to be provided tracking and analytics on your integration.

When you have your API key, you can include it in any API request by passing it as an additional parameter to any of the SDK functions:

```rust
use aori_rs::{get_quote, submit_swap, QuoteRequest, SwapRequest, AORI_API};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from your preferred method
    let api_key = std::env::var("AORI_API_KEY").ok();

    // Use it with any API call
    let quote = get_quote(&quote_request, Some(AORI_API), api_key.as_deref()).await?;

    // Then submit a swap with the same key
    let swap = submit_swap(&swap_request, Some(AORI_API), api_key.as_deref()).await?;

    Ok(())
}
```

You can also use API keys with WebSocket connections:

```rust
use aori_rs::{AoriWebSocket, AORI_WS_API};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize websocket with API key
    let api_key = std::env::var("AORI_API_KEY").ok();
    let ws = AoriWebSocket::new(Some(AORI_WS_API), api_key);

    ws.connect(|event| {
        println!("Received event: {:?}", event);
    }).await?;

    Ok(())
}
```

## API Reference

| Method | Endpoint                   | Description                      | Request Body     |
| ------ | -------------------------- | -------------------------------- | ---------------- |
| `GET`  | `/chains`                  | Get a list of supported chains   | -                |
| `POST` | `/quote`                   | Get a quote                      | `<QuoteRequest>` |
| `POST` | `/swap`                    | Execute Swap                     | `<SwapRequest>`  |
| `GET`  | `/data`                    | Query Historical Orders Database | -                |
| `GET`  | `/data/status/{orderHash}` | Get Swap Details/Status          | -                |
| `WS`   | `/stream`                  | Open a Websocket Connection      | -                |

### `/quote`

The quote endpoint acts as the primary endpoint for users to request quotes.

#### Example QuoteRequest

```bash
curl -X POST https://v3development.api.aori.io/quote \
-H "Content-Type: application/json" \
-H "x-api-key: your_api_key_here" \
-d '{
    "offerer": "0x0000000000000000000000000000000000000000",
    "recipient": "0x0000000000000000000000000000000000000000",
    "inputToken": "0x4200000000000000000000000000000000000006",
    "outputToken": "0xaf88d065e77c8cc2239327c5edb3a432268e5831",
    "inputAmount": "1000000000000000000",
    "inputChain": "base",
    "outputChain": "arbitrum"
}'
```

#### Example QuoteResponse

```json
{
  "orderHash": "0x9a3af...",
  "signingHash": "0xas23f...",
  "offerer": "0x0000000000000000000000000000000000000001",
  "recipient": "0x0000000000000000000000000000000000000001",
  "inputToken": "0x0000000000000000000000000000000000000002",
  "outputToken": "0x0000000000000000000000000000000000000003",
  "inputAmount": "1000000000000000000",
  "outputAmount": "1000000000000000000",
  "inputChain": "base",
  "outputChain": "arbitrum",
  "startTime": "1700000000",
  "endTime": "1700000010",
  "estimatedTime": 3000
}
```

### `/swap`

The swap endpoint acts as the primary endpoint for users to post signed orders for execution.

#### Example SwapRequest

```bash
curl -X POST https://api.aori.io/swap \
-H "Content-Type: application/json" \
-d '{
    "orderHash": "0x...",
    "signature": "0x..."
}'
```

#### Example SwapResponse

```json
{
  "orderHash": "0x9a3af...",
  "offerer": "0x0000000000000000000000000000000000000001",
  "recipient": "0x0000000000000000000000000000000000000001",
  "inputToken": "0x0000000000000000000000000000000000000002",
  "outputToken": "0x0000000000000000000000000000000000000003",
  "inputAmount": "1000000000000000000",
  "outputAmount": "1000000000000000000",
  "inputChain": "base",
  "outputChain": "arbitrum",
  "startTime": "1700000000",
  "endTime": "1700000010",
  "status": "pending",
  "createdAt": "1700000000"
}
```

### `/data`

The data endpoint acts as the primary endpoint for users to query historical orders.

#### Parameters

| Parameter     | Type           | Description                                                 |
| ------------- | -------------- | ----------------------------------------------------------- |
| order_hash    | String         | Hash of the order                                           |
| offerer       | String         | Address of the order creator                                |
| recipient     | String         | Address of the order recipient                              |
| input_token   | String         | Address of the token being sent                             |
| input_amount  | String         | Amount of input token                                       |
| output_token  | String         | Address of the token being received                         |
| output_amount | String         | Amount of output token                                      |
| input_chain   | String         | Chain key for the input token (e.g., "arbitrum")            |
| output_chain  | String         | Chain key for the output token (e.g., "base")               |
| src_tx        | Option<String> | Source chain transaction hash                               |
| dst_tx        | Option<String> | Destination chain transaction hash                          |
| status        | String         | Order status (Pending, Received, Completed, Failed)        |
| min_time      | i64            | Unix timestamp, start of filter range by created_at         |
| max_time      | i64            | Unix timestamp, end of filter range by created_at           |
| page          | i64            | Page number (1-x)                                           |
| limit         | i64            | Results per page (1-100)                                    |

## Chains

| Chain    | chainKey   | chainId | eid   | address                                      | vm  |
| -------- | ---------- | ------- | ----- | -------------------------------------------- | --- |
| Ethereum | `ethereum` | 1       | 30101 | `0xe8820573Bb2d748Dc86C381b2c4bC3cFdFabf30A` | EVM |
| Base     | `base`     | 8453    | 30184 | `0x21FC19BE519fB20e9182aDF3Ca0C2Ef625aB1647` | EVM |
| Arbitrum | `arbitrum` | 42161   | 30110 | `0x708a4498dA06b133f73Ee6107F1737015372cb76` | EVM |
| Optimism | `optimism` | 10      | 30111 | `0xbfd66f36aCa484802387a8e484BCe4630A1da764` | EVM |

## SDK Functions

| Function            | Description                                                    | Parameters                                                                                  | Return Type                                       |
| ------------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| `get_quote`         | Requests a quote for a token swap                              | `request: &QuoteRequest, base_url: Option<&str>, api_key: Option<&str>`                     | `Result<QuoteResponse>`                           |
| `sign_order`        | Signs an order using the provided private key                  | `quote_response: &QuoteResponse, private_key: &str`                                         | `Result<String>`                                  |
| `sign_readable_order` | Signs an order using EIP-712 typed data (for frontends)      | `quote_response: &QuoteResponse, private_key: &str, user_address: &str, base_url: Option<&str>` | `Result<(String, String)>`                       |
| `submit_swap`       | Submits a signed swap order to the Aori API.                   | `request: &SwapRequest, base_url: Option<&str>, api_key: Option<&str>`                      | `Result<SwapResponse>`                            |
| `get_order_status`  | Gets the current status of an order.                           | `order_hash: &str, base_url: Option<&str>, api_key: Option<&str>`                           | `Result<OrderStatus>`                             |
| `get_chains`        | Fetches the list of supported chains and their configurations. | `base_url: Option<&str>, api_key: Option<&str>`                                             | `Result<Vec<ChainInfo>>`                          |
| `poll_order_status` | Polls the status of an order until completion or timeout.      | `order_hash: &str, base_url: Option<&str>, api_key: Option<&str>, options: PollOrderStatusOptions` | `Result<OrderStatus>`                             |
| `get_order_details` | Fetches detailed information about an order.                   | `order_hash: &str, base_url: Option<&str>, api_key: Option<&str>`                           | `Result<OrderDetails>`                            |
| `query_orders`      | Queries orders with filtering criteria.                        | `base_url: &str, params: &QueryOrdersParams, api_key: Option<&str>`                        | `Result<QueryOrdersResponse>`                     |

# Examples

### Using API Keys with Environment Variables

This example demonstrates how to use API keys from environment variables:

```rust
use aori_rs::{
    get_quote, 
    submit_swap, 
    get_order_status,
    sign_order,
    QuoteRequest,
    SwapRequest,
    AORI_API
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = std::env::var("AORI_API_KEY").ok();
    
    // Create a quote request
    let quote_request = QuoteRequest {
        offerer: "0xYourAddress".to_string(),
        recipient: "0xYourAddress".to_string(),
        input_token: "0xInputTokenAddress".to_string(),
        output_token: "0xOutputTokenAddress".to_string(),
        input_amount: "1000000000000000000".to_string(), // 1 token with 18 decimals
        input_chain: "arbitrum".to_string(),
        output_chain: "base".to_string(),
    };
    
    // Include API key in all requests
    let quote = get_quote(&quote_request, Some(AORI_API), api_key.as_deref()).await?;
    println!("Quote received: {:?}", quote);
    
    // Sign the quote
    let private_key = "your_private_key_here";
    let signature = sign_order(&quote, private_key)?;
    
    // Submit the swap with API key
    let swap_response = submit_swap(&SwapRequest {
        order_hash: quote.order_hash,
        signature,
    }, Some(AORI_API), api_key.as_deref()).await?;
    
    // Check order status with API key
    let status = get_order_status(&swap_response.order_hash, Some(AORI_API), api_key.as_deref()).await?;
    println!("Order status: {:?}", status);

    Ok(())
}
```

### Executing an Order with Private Key Signing

This example demonstrates how to use the SDK with private key signing for backend applications:

```rust
use aori_rs::{
    get_quote,
    sign_order,
    submit_swap,
    get_order_status,
    poll_order_status,
    PollOrderStatusOptions,
    QuoteRequest,
    SwapRequest,
    AORI_API
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variables or a secured source
    let api_key = std::env::var("AORI_API_KEY").ok();
    let private_key = std::env::var("PRIVATE_KEY")?;

    // 1. Get the quote first
    let quote_request = QuoteRequest {
        offerer: "0x1234567890123456789012345678901234567890".to_string(),
        recipient: "0x1234567890123456789012345678901234567890".to_string(),
        input_token: "0x4200000000000000000000000000000000000006".to_string(), // WETH on Base
        output_token: "0xaf88d065e77c8cc2239327c5edb3a432268e5831".to_string(), // USDC on Arbitrum
        input_amount: "1000000000000000000".to_string(), // 1 WETH
        input_chain: "base".to_string(),
        output_chain: "arbitrum".to_string(),
    };

    let quote = get_quote(&quote_request, Some(AORI_API), api_key.as_deref()).await?;
    println!("Quote received: {:?}", quote);

    // 2. Sign the order using private key
    let signature = sign_order(&quote, &private_key)?;

    // 3. Submit the swap with signature (including API key)
    let swap_request = SwapRequest {
        order_hash: quote.order_hash.clone(),
        signature,
    };

    let swap_response = submit_swap(&swap_request, Some(AORI_API), api_key.as_deref()).await?;
    println!("Swap submitted successfully: {:?}", swap_response);

    // 4. Poll for order status with callbacks
    let options = PollOrderStatusOptions {
        on_status_change: Some(|status| {
            println!("Status changed: {:?}", status);
        }),
        on_complete: Some(|status| {
            println!("Order completed: {:?}", status);
        }),
        on_error: Some(|error| {
            eprintln!("Polling error: {:?}", error);
        }),
        interval: Some(5000), // 5 seconds
        timeout: Some(300000), // 5 minutes
    };

    let final_status = poll_order_status(
        &swap_response.order_hash,
        Some(AORI_API),
        api_key.as_deref(),
        options
    ).await?;

    println!("Final order status: {:?}", final_status);

    Ok(())
}
```

### Real-time Order Monitoring with WebSocket

This example demonstrates WebSocket usage for real-time order monitoring:

```rust
use aori_rs::{AoriWebSocket, WSEventType, AORI_WS_API};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("AORI_API_KEY").ok();
    let ws = AoriWebSocket::new(Some(AORI_WS_API), api_key);
    
    ws.connect(|event| {
        println!("Received WebSocket event: {:?}", event);
        
        match event.event_type {
            WSEventType::Created => {
                println!("🆕 New order created: {}", event.order.order_hash);
            },
            WSEventType::Received => {
                println!("📥 Order received: {} - Tx: {}", 
                    event.order.order_hash, 
                    event.order.src_tx
                );
            },
            WSEventType::Completed => {
                println!("✅ Order completed: {} - Tx: {}", 
                    event.order.order_hash, 
                    event.order.dst_tx
                );
            },
            WSEventType::Failed => {
                println!("❌ Order failed: {}", event.order.order_hash);
            },
        }
    }).await?;

    Ok(())
}
```

### Querying Order History

This example demonstrates how to query historical orders with filtering:

```rust
use aori_rs::{query_orders, QueryOrdersParams, AORI_API};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("AORI_API_KEY").ok();
    
    let params = QueryOrdersParams {
        offerer: Some("0x1234567890123456789012345678901234567890".to_string()),
        input_chain: Some("ethereum".to_string()),
        output_chain: Some("base".to_string()),
        status: Some("completed".to_string()),
        page: Some(1),
        limit: Some(10),
        ..Default::default()
    };

    let results = query_orders(AORI_API, &params, api_key.as_deref()).await?;
    
    println!("Found {} orders", results.orders.len());
    println!("Page {} of {} (Total: {} orders)", 
        results.pagination.current_page,
        results.pagination.total_pages,
        results.pagination.total_records
    );
    
    for order in results.orders {
        println!("Order: {} - Status: {} - Amount: {} {}", 
            order.order_hash, 
            order.status,
            order.input_amount,
            order.input_chain
        );
    }

    Ok(())
}
```

## License

This project is released under the [MIT License](LICENSE).
