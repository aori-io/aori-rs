use aori_rs::swap::SwapClient;
use aori_rs::types::SwapGetRequest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = SwapClient::new();
    
    // Example: Get quote for swapping 1 ETH for USDC
    let request = SwapGetRequest {
        input_token: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(), // WETH
        input_amount: "1000000000000000000".to_string(), // 1 ETH (18 decimals)
        input_chain_id: 1, // Ethereum mainnet
        output_token: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(), // USDC
        output_chain_id: None, // or 1 for output on same chain (Ethereum)
    };

    let quote = client.get_quote(request).await?;
    println!("Received quote: {:?}", quote);

    Ok(())
} 