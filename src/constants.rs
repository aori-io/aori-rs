use crate::types::ChainInfo;
use std::collections::HashMap;

//////////////////////////////////////////////////////////////
//                      SUPPORTED CHAINS
//////////////////////////////////////////////////////////////

pub fn get_chains_map() -> HashMap<String, ChainInfo> {
    let mut chains = HashMap::new();
    
    chains.insert("ethereum".to_string(), ChainInfo {
        chain_key: "ethereum".to_string(),
        chain_id: 1,
        eid: 30101,
        address: "0x3C5ee8Ec2E0174cE1B34f140F37C032e43ef41b6".to_string(),
        blocktime: 12,
    });
    
    chains.insert("base".to_string(), ChainInfo {
        chain_key: "base".to_string(),
        chain_id: 8453,
        eid: 30184,
        address: "0x69F05F7Fb4D7E3C11943392BE5254c9c19c01647".to_string(),
        blocktime: 2,
    });
    
    chains.insert("arbitrum".to_string(), ChainInfo {
        chain_key: "arbitrum".to_string(),
        chain_id: 42161,
        eid: 30110,
        address: "0x629B94B73229a22051dB15A3122c426aa68D0A87".to_string(),
        blocktime: 1,
    });
    
    chains.insert("optimism".to_string(), ChainInfo {
        chain_key: "optimism".to_string(),
        chain_id: 10,
        eid: 30111,
        address: "0x684986544162a2c4cE4a6879981a4969b2c19E92".to_string(),
        blocktime: 2,
    });
    
    chains
}

/// Helper function to get chain info by chain ID
pub fn get_chain_info_by_id(chain_id: u32) -> Option<ChainInfo> {
    get_chains_map().values().find(|chain| chain.chain_id == chain_id).cloned()
}

/// Helper function to get chain info by chain key
pub fn get_chain_info_by_key(chain_key: &str) -> Option<ChainInfo> {
    get_chains_map().get(&chain_key.to_lowercase()).cloned()
}

/// Helper function to get chain info by EID
pub fn get_chain_info_by_eid(eid: u32) -> Option<ChainInfo> {
    get_chains_map().values().find(|chain| chain.eid == eid).cloned()
}

////////////////////////////////////////////////////////////////
//                      WEBSOCKET URLS
////////////////////////////////////////////////////////////////

pub const AORI_WS_API: &str = "wss://api.aori.io";
pub const AORI_WS_DEVELOPMENT_API: &str = "wss://dev.api.aori.io";

////////////////////////////////////////////////////////////////
//                      HTTP POST URLS
////////////////////////////////////////////////////////////////

/// Main Aori API for facilitating CRUD operations
pub const AORI_API: &str = "https://api.aori.io";
pub const AORI_DEVELOPMENT_API: &str = "https://dev.api.aori.io";
