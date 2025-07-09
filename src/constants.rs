use crate::types::ChainInfo;
use std::collections::HashMap;

//////////////////////////////////////////////////////////////
//                      SUPPORTED CHAINS
//////////////////////////////////////////////////////////////

pub fn get_chains_map() -> HashMap<String, ChainInfo> {
    let mut chains = HashMap::new();

    chains.insert(
        "ethereum".to_string(),
        ChainInfo {
            chain_key: "ethereum".to_string(),
            chain_id: 1,
            eid: 30101,
            address: "0x98AD96Ef787ba5180814055039F8E37d98ADea63".to_string(),
        },
    );

    chains.insert(
        "base".to_string(),
        ChainInfo {
            chain_key: "base".to_string(),
            chain_id: 8453,
            eid: 30184,
            address: "0xFfe691A6dDb5D2645321e0a920C2e7Bdd00dD3D8".to_string(),
        },
    );

    chains.insert(
        "arbitrum".to_string(),
        ChainInfo {
            chain_key: "arbitrum".to_string(),
            chain_id: 42161,
            eid: 30110,
            address: "0xFfe691A6dDb5D2645321e0a920C2e7Bdd00dD3D8".to_string(),
        },
    );

    chains.insert(
        "optimism".to_string(),
        ChainInfo {
            chain_key: "optimism".to_string(),
            chain_id: 10,
            eid: 30111,
            address: "0xFfe691A6dDb5D2645321e0a920C2e7Bdd00dD3D8".to_string(),
        },
    );

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
//                            URLS
////////////////////////////////////////////////////////////////

pub const AORI_API: &str = "https://api.aori.io";
pub const AORI_WS_API: &str = "wss://api.aori.io";
