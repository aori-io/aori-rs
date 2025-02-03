use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////
//                      Aori Swap Structs
////////////////////////////////////////////////////////////////

/// Aori.sol Order Struct
// struct Order {
//   address offerer;
//   address recipient;
//   address srcInputToken;
//   uint256 srcInputAmount;
//   address dstOutputToken;
//   uint256 dstOutputAmount;
//   uint256 dstChainId;
//   uint256 startTime;
//   uint256 endTime;
// }

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