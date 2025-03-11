use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////
//                      Aori Swap Structs
////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    pub chain_key: String,
    pub chain_id: u32,
    pub eid: i32,
    pub address: String,
    pub blocktime: &'static str,
  }
  
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub output_token: String,
    pub input_amount: u64,
    pub input_chain: String,
    pub output_chain: String, 
}
  
#[derive(Debug, Deserialize)]
pub struct QuoteResponse {
    pub order_hash: String,
    pub signing_hash: String,
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub output_token: String,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_chain: String,
    pub output_chain: String,
    pub start_time: u64,
    pub end_time: u64,
    pub estimated_time: u64,
    pub exclusive_solver: String,
    pub exclusive_solver_duration: u64,
}
  
  #[derive(Debug, Deserialize, Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct SwapRequest {
    pub order_hash: String,
    pub signature: String,
  }
  
  #[derive(Debug, Deserialize, Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct SignerType {
    pub private_key: &'static str,
  }
  #[derive(Debug, Deserialize, Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct SwapResponse {
    pub order_hash: String,
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub output_token: String,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_chain: String,
    pub output_chain: String,
    pub start_time: u64,
    pub end_time: u64,
    pub status: String,
    pub created_at: u64,
  }

  #[derive(Default, Debug, Deserialize, Serialize, Clone)]
  #[serde(rename_all = "camelCase")]
  pub struct OrderRecord {
    pub order_hash: String,
    pub offerer: String,
    pub recipient: String,
    pub input_token: String,
    pub output_token: String,
    pub input_value_usd: String,
    pub output_amount: String,
    pub output_value_usd: String,
    pub input_chain: String,
    pub output_chain: String,
    pub start_time: u64,
    pub end_time: u64,
    pub src_tx: Option<String>,
    pub dst_tx: Option<String>,
    pub status: String,
    pub created_at: u64,
    pub received_at: Option<u64>,
    pub filled_at: Option<u64>,
    pub confirmed_at: Option<u64>,
    pub failed_at: Option<u64>,
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