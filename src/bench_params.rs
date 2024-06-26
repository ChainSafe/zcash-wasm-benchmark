use std::str::FromStr;

/// Global params shared between the benchmarks
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct BenchParams {
    pub network: Network,
    pub pool: ShieldedPool,
    pub lightwalletd_url: String,
    pub start_block: u32,
    pub end_block: u32,
    pub block_batch_size: u32,
}

#[wasm_bindgen]
impl BenchParams {
    #[wasm_bindgen(constructor)]
    pub fn new(
        network: String,
        pool: String,
        lightwalletd_url: String,
        start_block: u32,
        end_block: u32,
        block_batch_size: u32,
    ) -> BenchParams {
        BenchParams {
            network: Network::from_str(&network).unwrap(),
            pool: ShieldedPool::from_str(&pool).unwrap(),
            lightwalletd_url,
            start_block,
            end_block,
            block_batch_size,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub enum Network {
    Mainnet,
    Testnet,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum ShieldedPool {
    Sapling,
    Orchard,
    Both,
}

impl ShieldedPool {
    pub fn sync_sapling(&self) -> bool {
        match self {
            ShieldedPool::Sapling => true,
            ShieldedPool::Orchard => false,
            ShieldedPool::Both => true,
        }
    }

    pub fn sync_orchard(&self) -> bool {
        match self {
            ShieldedPool::Sapling => false,
            ShieldedPool::Orchard => true,
            ShieldedPool::Both => true,
        }
    }
}

impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mainnet" => Ok(Network::Mainnet),
            "testnet" => Ok(Network::Testnet),
            _ => Err(format!("Invalid network: {}", s)),
        }
    }
}

impl FromStr for ShieldedPool {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sapling" => Ok(ShieldedPool::Sapling),
            "orchard" => Ok(ShieldedPool::Orchard),
            "both" => Ok(ShieldedPool::Both),
            _ => Err(format!("Invalid pool: {}", s)),
        }
    }
}
