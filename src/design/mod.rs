use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod presets;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainDesign {
    pub design_name: String,
    pub description: String,
    pub consensus: ConsensusParameters,
    pub optimization: OptimizationCriteria,
    pub network: NetworkParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusParameters {
    pub scrypt_n: u64,
    pub scrypt_r: u32,
    pub scrypt_p: u32,
    pub scrypt_len: usize,
    pub target_block_time_secs: u64,
    pub max_payloads_per_block: usize,
    pub payload_size_bytes: usize,
    pub blocks_per_year: u64,
    pub difficulty_adjustment_period_blocks: u64,
    pub difficulty_vote_window_blocks: u64,
    pub block_reward: u64,
    pub reward_decimals: u32,
    pub max_supply: u64,
    pub prune_depth_blocks: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationCriteria {
    pub throughput_weight: f64,
    pub latency_weight: f64,
    pub disk_efficiency_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkParameters {
    pub p2p_protocol_version: String,
    pub listen_port: u16,
    pub gossipsub_heartbeat_secs: u64,
}

#[derive(Debug, Error)]
pub enum DesignError {
    #[error("scrypt_n must be a power of 2 between 1024 and 1048576, got {0}")]
    InvalidScryptN(u64),
    #[error("target_block_time_secs must be between 10 and 3600, got {0}")]
    InvalidBlockTime(u64),
    #[error("max_payloads_per_block must be between 1 and 65536, got {0}")]
    InvalidMaxPayloads(usize),
    #[error("payload_size_bytes must be between 16 and 4096, got {0}")]
    InvalidPayloadSize(usize),
    #[error("optimization weights must sum to 1.0, got {0}")]
    InvalidWeights(f64),
    #[error("design_name must be snake_case, got '{0}'")]
    InvalidName(String),
}

impl BlockchainDesign {
    pub fn validate(&self) -> Result<(), DesignError> {
        self.consensus.validate()?;
        self.optimization.validate()?;
        self.validate_name()?;
        Ok(())
    }

    fn validate_name(&self) -> Result<(), DesignError> {
        if self.design_name.is_empty() {
            return Err(DesignError::InvalidName(self.design_name.clone()));
        }
        if self.design_name.contains(' ') || self.design_name.contains('-') {
            return Err(DesignError::InvalidName(self.design_name.clone()));
        }
        Ok(())
    }
}

impl ConsensusParameters {
    pub fn validate(&self) -> Result<(), DesignError> {
        if !is_power_of_two(self.scrypt_n) || self.scrypt_n < 1024 || self.scrypt_n > 1_048_576 {
            return Err(DesignError::InvalidScryptN(self.scrypt_n));
        }
        if self.target_block_time_secs < 10 || self.target_block_time_secs > 3600 {
            return Err(DesignError::InvalidBlockTime(self.target_block_time_secs));
        }
        if self.max_payloads_per_block == 0 || self.max_payloads_per_block > 65_536 {
            return Err(DesignError::InvalidMaxPayloads(self.max_payloads_per_block));
        }
        if self.payload_size_bytes < 16 || self.payload_size_bytes > 4096 {
            return Err(DesignError::InvalidPayloadSize(self.payload_size_bytes));
        }
        Ok(())
    }
}

impl OptimizationCriteria {
    pub fn validate(&self) -> Result<(), DesignError> {
        let sum = self.throughput_weight + self.latency_weight + self.disk_efficiency_weight;
        if (sum - 1.0).abs() > 0.01 {
            return Err(DesignError::InvalidWeights(sum));
        }
        Ok(())
    }
}

fn is_power_of_two(n: u64) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

impl Default for BlockchainDesign {
    fn default() -> Self {
        Self {
            design_name: "default_chain".to_string(),
            description: "Default blockchain configuration".to_string(),
            consensus: ConsensusParameters {
                scrypt_n: 32_768,
                scrypt_r: 8,
                scrypt_p: 1,
                scrypt_len: 32,
                target_block_time_secs: 120,
                max_payloads_per_block: 1024,
                payload_size_bytes: 256,
                blocks_per_year: 262_800,
                difficulty_adjustment_period_blocks: 262_800,
                difficulty_vote_window_blocks: 1000,
                block_reward: 500_000_000,
                reward_decimals: 9,
                max_supply: 21_000_000_000_000_000,
                prune_depth_blocks: 1000,
            },
            optimization: OptimizationCriteria {
                throughput_weight: 0.33,
                latency_weight: 0.33,
                disk_efficiency_weight: 0.34,
            },
            network: NetworkParameters {
                p2p_protocol_version: "0.1.0".to_string(),
                listen_port: 6262,
                gossipsub_heartbeat_secs: 10,
            },
        }
    }
}
