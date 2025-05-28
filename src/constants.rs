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
        address: "0xAC23dd76E55E15be6bB10057c37fCF307cd0bfD5".to_string(),
    });
    
    chains.insert("base".to_string(), ChainInfo {
        chain_key: "base".to_string(),
        chain_id: 8453,
        eid: 30184,
        address: "0xf0304563e05B1E2Bc3De8DC80185E8Ca2940CA04".to_string(),
    });
    
    chains.insert("arbitrum".to_string(), ChainInfo {
        chain_key: "arbitrum".to_string(),
        chain_id: 42161,
        eid: 30110,
        address: "0x83dE87A541613B50263A216a210B8fcdfd5DBc2C".to_string(),
    });
    
    chains.insert("optimism".to_string(), ChainInfo {
        chain_key: "optimism".to_string(),
        chain_id: 10,
        eid: 30111,
        address: "0x62438859025E2DDd589F21c109de4C77EE308C91".to_string(),
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
//                            URLS
////////////////////////////////////////////////////////////////

pub const AORI_API: &str = "https://api.aori.io";
pub const AORI_WS_API: &str = "wss://api.aori.io";
