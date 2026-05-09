use etch_gen::design::{BlockchainDesign, ConsensusParameters, OptimizationCriteria, NetworkParameters};
use etch_gen::llm::DesignParser;

#[test]
fn test_parser_valid_json() {
    let json = r#"{
        "design_name": "test_chain",
        "description": "A test blockchain",
        "consensus": {
            "scrypt_n": 4096,
            "scrypt_r": 8,
            "scrypt_p": 1,
            "scrypt_len": 32,
            "target_block_time_secs": 60,
            "max_payloads_per_block": 512,
            "payload_size_bytes": 128,
            "blocks_per_year": 525600,
            "difficulty_adjustment_period_blocks": 525600,
            "difficulty_vote_window_blocks": 1000,
            "block_reward": 500000000,
            "reward_decimals": 9,
            "max_supply": 21000000000000000,
            "prune_depth_blocks": 1000
        },
        "optimization": {
            "throughput_weight": 0.5,
            "latency_weight": 0.3,
            "disk_efficiency_weight": 0.2
        },
        "network": {
            "p2p_protocol_version": "0.1.0",
            "listen_port": 6262,
            "gossipsub_heartbeat_secs": 10
        }
    }"#;

    let parser = DesignParser::new();
    let design = parser.parse(json).expect("should parse valid JSON");
    assert_eq!(design.design_name, "test_chain");
    assert_eq!(design.consensus.scrypt_n, 4096);
    assert_eq!(design.consensus.target_block_time_secs, 60);
}

#[test]
fn test_parser_json_in_markdown() {
    let markdown = r#"Here is the design:

```json
{
    "design_name": "markdown_chain",
    "description": "From markdown",
    "consensus": {
        "scrypt_n": 2048,
        "scrypt_r": 8,
        "scrypt_p": 1,
        "scrypt_len": 32,
        "target_block_time_secs": 30,
        "max_payloads_per_block": 1024,
        "payload_size_bytes": 256,
        "blocks_per_year": 1051200,
        "difficulty_adjustment_period_blocks": 1051200,
        "difficulty_vote_window_blocks": 1000,
        "block_reward": 500000000,
        "reward_decimals": 9,
        "max_supply": 21000000000000000,
        "prune_depth_blocks": 1000
    },
    "optimization": {
        "throughput_weight": 0.34,
        "latency_weight": 0.33,
        "disk_efficiency_weight": 0.33
    },
    "network": {
        "p2p_protocol_version": "0.1.0",
        "listen_port": 6262,
        "gossipsub_heartbeat_secs": 10
    }
}
```

Hope this helps!"#;

    let parser = DesignParser::new();
    let design = parser.parse(markdown).expect("should extract JSON from markdown");
    assert_eq!(design.design_name, "markdown_chain");
}

#[test]
fn test_design_validation_rejects_invalid_scrypt_n() {
    let design = BlockchainDesign {
        design_name: "bad_chain".to_string(),
        description: "test".to_string(),
        consensus: ConsensusParameters {
            scrypt_n: 1000,
            scrypt_r: 8,
            scrypt_p: 1,
            scrypt_len: 32,
            target_block_time_secs: 120,
            max_payloads_per_block: 1024,
            payload_size_bytes: 256,
            blocks_per_year: 262800,
            difficulty_adjustment_period_blocks: 262800,
            difficulty_vote_window_blocks: 1000,
            block_reward: 500000000,
            reward_decimals: 9,
            max_supply: 21000000000000000,
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
    };

    assert!(design.validate().is_err(), "should reject non-power-of-2 scrypt_n");
}

#[test]
fn test_design_validation_rejects_bad_weights() {
    let design = BlockchainDesign {
        design_name: "bad_weights".to_string(),
        description: "test".to_string(),
        consensus: ConsensusParameters {
            scrypt_n: 2048,
            scrypt_r: 8,
            scrypt_p: 1,
            scrypt_len: 32,
            target_block_time_secs: 120,
            max_payloads_per_block: 1024,
            payload_size_bytes: 256,
            blocks_per_year: 262800,
            difficulty_adjustment_period_blocks: 262800,
            difficulty_vote_window_blocks: 1000,
            block_reward: 500000000,
            reward_decimals: 9,
            max_supply: 21000000000000000,
            prune_depth_blocks: 1000,
        },
        optimization: OptimizationCriteria {
            throughput_weight: 0.5,
            latency_weight: 0.5,
            disk_efficiency_weight: 0.5,
        },
        network: NetworkParameters {
            p2p_protocol_version: "0.1.0".to_string(),
            listen_port: 6262,
            gossipsub_heartbeat_secs: 10,
        },
    };

    assert!(design.validate().is_err(), "should reject weights that don't sum to 1.0");
}

#[test]
fn test_design_validation_rejects_invalid_name() {
    let mut design = BlockchainDesign::default();
    design.design_name = "bad-name".to_string();
    assert!(design.validate().is_err(), "should reject kebab-case name");
}
