use aori_rs_sdk::swap;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use swap::{AoriWebSocket, AoriWebSocketClient, WebSocketCloseEvent, WebSocketError};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use tokio::time::sleep;
use tokio_tungstenite::{
    accept_async, tungstenite::protocol::Message,
};

async fn run_mock_server(shutdown_signal: oneshot::Receiver<()>) {
    // Set up the server address
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Mock WebSocket server listening on: {}", addr);

    // Create a future that completes when the shutdown signal is received
    let shutdown = async {
        let _ = shutdown_signal.await;
        println!("Shutting down mock server");
    };

    // Create a future that accepts connections
    let server = async {
        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream.peer_addr().unwrap();
            println!("Peer connected: {}", peer);
            
            // Spawn a new task for each connection
            tokio::spawn(handle_connection(stream, peer));
        }
    };

    // Run the server until the shutdown signal is received
    tokio::select! {
        _ = shutdown => {},
        _ = server => {},
    }
}

async fn handle_connection(stream: TcpStream, peer: SocketAddr) {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");
    println!("New WebSocket connection: {}", peer);

    // Send a welcome message
    ws_stream.send(Message::Text(r#"{"jsonrpc":"2.0","result":"connected","id":0}"#.to_string())).await.unwrap();
    
    // Process incoming messages
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received from {}: {}", peer, text);
                
                // Parse the message to determine response
                if text.contains("subscribe") {
                    // Send a subscription confirmation
                    ws_stream.send(Message::Text(r#"{"jsonrpc":"2.0","result":"subscribed","id":1}"#.to_string())).await.unwrap();
                    
                    // Send a mock order after subscription
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    ws_stream.send(Message::Text(r#"{"status":"pending","order_hash":"0x123","src_chain_id":1,"dst_chain_id":10}"#.to_string())).await.unwrap();
                    
                    // Send a filled order after 3 seconds
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    ws_stream.send(Message::Text(r#"{"status":"filled","order_hash":"0x123","src_chain_id":1,"dst_chain_id":10,"dst_tx":"0xabc"}"#.to_string())).await.unwrap();
                } else if text.contains("ping") {
                    // Respond to ping
                    ws_stream.send(Message::Text(r#"{"jsonrpc":"2.0","result":"pong","id":2}"#.to_string())).await.unwrap();
                }
            }
            Ok(Message::Ping(data)) => {
                // Respond to ping with pong
                ws_stream.send(Message::Pong(data)).await.unwrap();
            }
            Err(e) => {
                println!("Error from {}: {}", peer, e);
                break;
            }
            _ => {}
        }
    }
    
    println!("Connection closed: {}", peer);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a channel to signal the server to shut down
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    
    // Start the mock server
    let server_handle = tokio::spawn(run_mock_server(shutdown_rx));
    
    // Wait for the server to start
    sleep(Duration::from_millis(500)).await;
    
    // Create the WebSocket options with callbacks
    let options = AoriWebSocket {
        on_message: Some(Arc::new(|order| {
            println!("Received order: {:?}", order);
        })),
        on_connect: Some(Arc::new(|| {
            println!("Connected to WebSocket server!");
        })),
        on_disconnect: Some(Arc::new(|event: WebSocketCloseEvent| {
            println!("Disconnected: code={}, reason={}", event.code, event.reason);
        })),
        on_error: Some(Arc::new(|error: WebSocketError| {
            println!("WebSocket error: {}", error.message);
        })),
    };

    // Create the client with the local server URL
    let client = AoriWebSocketClient::new(
        Some("ws://127.0.0.1:8080".to_string()),
        options
    );

    // Connect to the server
    println!("Connecting to WebSocket server...");
    client.connect().await?;

    // Wait a bit to ensure connection is established
    sleep(Duration::from_secs(2)).await;

    // Check if connected
    if client.is_connected().await {
        println!("Successfully connected!");

        // Send a test message
        println!("Sending test message...");
        match client
            .send_message(r#"{"jsonrpc":"2.0","method":"ping","params":[],"id":2}"#.to_string())
            .await
        {
            Ok(_) => println!("Test message sent successfully"),
            Err(e) => println!("Failed to send test message: {}", e),
        }
    } else {
        println!("Not connected.");
    }

    // Wait for some messages
    println!("Waiting for messages for 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    // Disconnect
    println!("Disconnecting...");
    client.disconnect().await?;

    // Shut down the mock server
    let _ = shutdown_tx.send(());
    
    // Wait for the server to shut down
    let _ = server_handle.await;

    println!("Test completed.");
    Ok(())
}
