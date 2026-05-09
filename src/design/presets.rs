use crate::design::{BlockchainDesign, ConsensusParameters, OptimizationCriteria, NetworkParameters};

pub fn mainnet() -> BlockchainDesign {
    BlockchainDesign {
        design_name: "mainnet".to_string(),
        description: "Etch mainnet configuration".to_string(),
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

pub fn fast_test() -> BlockchainDesign {
    BlockchainDesign {
        design_name: "fast_test".to_string(),
        description: "Fast test configuration with easy difficulty".to_string(),
        consensus: ConsensusParameters {
            scrypt_n: 1_024,
            scrypt_r: 1,
            scrypt_p: 1,
            scrypt_len: 32,
            target_block_time_secs: 10,
            max_payloads_per_block: 100,
            payload_size_bytes: 256,
            blocks_per_year: 100,
            difficulty_adjustment_period_blocks: 100,
            difficulty_vote_window_blocks: 10,
            block_reward: 50,
            reward_decimals: 0,
            max_supply: 21_000_000,
            prune_depth_blocks: 100,
        },
        optimization: OptimizationCriteria {
            throughput_weight: 0.5,
            latency_weight: 0.3,
            disk_efficiency_weight: 0.2,
        },
        network: NetworkParameters {
            p2p_protocol_version: "0.1.0".to_string(),
            listen_port: 16262,
            gossipsub_heartbeat_secs: 1,
        },
    }
}
