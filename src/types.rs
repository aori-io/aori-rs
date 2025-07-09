use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct AoriOrder {
    offerer: String,
    recipient: String,
    input_token: String,
    input_amount: String,
    output_token: String,
    output_amount: String,
    start_time: u32,
    end_time: u32,
    src_eid: u32,
    dst_eid: u32,
}

////////////////////////////////////////////////////////////////
//                 /swap GET request -> response
////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapGetRequest {
    pub input_token: String,
    pub input_amount: String,
    pub input_chain_id: u32,
    pub output_token: String,
    pub output_chain_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapGetResponse {
    pub input_token: String,
    pub input_amount: String,
    pub input_chain_id: u32,
    pub output_token: String,
    pub output_amount: String,
    pub chain_id: u32,
    pub output_chain_id: u32,
    pub calldata: String,
}

////////////////////////////////////////////////////////////////
//                 /swap POST request -> response
////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapPostRequest {
    pub input_token: String,
    pub input_amount: String,
    pub input_chain_id: u32,
    pub output_token: String,
    pub output_chain_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapPostResponse {
    pub input_token: String,
    pub input_amount: String,
    pub input_chain_id: u32,
    pub output_token: String,
    pub output_amount: String,
    pub chain_id: u32,
    pub output_chain_id: u32,
    pub calldata: String,
}

//========================================================
//        Contract Compliant Order Struct Interface
//========================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub output_token: String,
    pub input_amount: String,
    pub input_chain: String,
    pub output_chain: String,
    pub start_time: u64,
    pub end_time: u64,
    pub src_eid: u32,
    pub dst_eid: u32,
}

//========================================================
//          Quote Request and Response Interfaces
//========================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub output_token: String,
    pub input_amount: String,
    pub input_chain: String,
    pub output_chain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub order_hash: String,
    pub signing_hash: String,
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub output_token: String,
    pub input_amount: String,
    pub output_amount: String,
    pub input_chain: String,
    pub output_chain: String,
    pub start_time: String,
    pub end_time: String,
    pub estimated_time: String,
}

//========================================================
//            Swap Request and Response Interfaces
//========================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapRequest {
    pub order_hash: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapResponse {
    pub order_hash: String,
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub output_token: String,
    pub input_amount: String,
    pub output_amount: String,
    pub input_chain: String,
    pub output_chain: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub created_at: u64,
}

//========================================================
//                Chain Info Interface
//========================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    pub chain_key: String,
    pub chain_id: u32,
    pub eid: u32,
    pub address: String,
}

//============================================
//          Order Status Interfaces
//=============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "status")]
pub enum OrderStatus {
    Pending(OrderPendingStatus),
    Received(OrderTxStatus),
    Completed(OrderTxStatus),
    Failed(OrderFailedStatus),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPendingStatus {
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderTxStatus {
    pub tx_hash: String,
    pub tx_url: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderFailedStatus {
    pub error: String,
    pub timestamp: u64,
}

//============================================
//         Order Details Interfaces
//=============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderEvent {
    pub event_type: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    pub order_hash: String,
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub input_amount: String,
    pub input_chain: String,
    pub input_token_value_usd: String,
    pub output_token: String,
    pub output_amount: String,
    pub output_chain: String,
    pub output_token_value_usd: String,
    pub start_time: i64,
    pub end_time: i64,
    pub src_tx: Option<String>,
    pub dst_tx: Option<String>,
    pub timestamp: i64,
    pub events: Vec<OrderEvent>,
}

//========================================================
//            Query Orders Interfaces
//========================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrdersParams {
    pub order_hash: Option<String>,
    pub offerer: Option<String>,
    pub recipient: Option<String>,
    pub input_token: Option<String>,
    pub input_chain: Option<String>,
    pub output_token: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub output_chain: Option<String>,
    pub srctx: Option<String>,
    pub dst_tx: Option<String>,
    pub status: Option<String>,
    pub min_time: Option<i64>,
    pub max_time: Option<i64>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderQueryResult {
    pub order_hash: String,
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub input_amount: String,
    pub input_chain: String,
    pub input_token_value_usd: String,
    pub output_token: String,
    pub output_amount: String,
    pub output_chain: String,
    pub output_token_value_usd: String,
    pub start_time: i64,
    pub end_time: i64,
    pub src_tx: Option<String>,
    pub dst_tx: Option<String>,
    pub timestamp: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMetadata {
    pub current_page: i64,
    pub limit: i64,
    pub total_records: i64,
    pub total_pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOrdersResponse {
    pub orders: Vec<OrderQueryResult>,
    pub pagination: PaginationMetadata,
}

//========================================================
//            WebSocket Interfaces
//========================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionParams {
    pub order_hash: Option<String>,
    pub offerer: Option<String>,
    pub recipient: Option<String>,
    pub input_token: Option<String>,
    pub input_chain: Option<String>,
    pub output_token: Option<String>,
    pub output_chain: Option<String>,
    pub event_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WSEventType {
    Created,
    Received,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSOrder {
    pub order_hash: String,
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub input_amount: String,
    pub input_chain: String,
    pub output_token: String,
    pub output_amount: String,
    pub output_chain: String,
    pub start_time: u64,
    pub src_tx: String,
    pub dst_tx: String,
    pub end_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WSEvent {
    pub event_type: WSEventType,
    pub timestamp: u64,
    pub order: WSOrder,
}

//========================================================
//                       Error Types
//========================================================

#[derive(Debug, thiserror::Error)]
pub enum AoriError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Chain not found: {0}")]
    ChainNotFound(String),
}

pub type Result<T> = std::result::Result<T, AoriError>;
