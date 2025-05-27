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
        address: "0xeA5c82C81CCc0ba69616c6eae40A6EC7F7794d87".to_string(),
    });
    
    chains.insert("base".to_string(), ChainInfo {
        chain_key: "base".to_string(),
        chain_id: 8453,
        eid: 30184,
        address: "0xBF693fcE30E7B08965E10A7ECddc92818d6a2a1e".to_string(),
    });
    
    chains.insert("arbitrum".to_string(), ChainInfo {
        chain_key: "arbitrum".to_string(),
        chain_id: 42161,
        eid: 30110,
        address: "0x437266584AdEae66F0edF0B97d14F399C8463731".to_string(),
    });
    
    chains.insert("optimism".to_string(), ChainInfo {
        chain_key: "optimism".to_string(),
        chain_id: 10,
        eid: 30111,
        address: "0xAA8Ec1a2C2814aAc925107e2b3c94ee0E8367ab5".to_string(),
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
