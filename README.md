# etch-gen

AI-powered blockchain generator using the [Etch](https://github.com/awdemos/etch) architecture.

Describe your desired blockchain in natural language, and etch-gen consults a local LLM to design the parameters, then generates a complete, standalone Rust crate that compiles and runs.

## What It Does

- Takes a natural language prompt (e.g., "fast block times for testing")
- Consults a local OpenAI-compatible LLM API to derive consensus parameters
- Generates a complete Rust blockchain crate following Etch's architecture
- The generated crate compiles with `cargo check` out of the box

## What It Doesn't Do

- Does not guarantee the generated blockchain is secure or well-parameterized
- Does not run the generated blockchain (that's up to you)
- Does not train models or do on-device inference (requires a running LLM API)

## Prerequisites

- Rust toolchain (1.75+)
- A running OpenAI-compatible LLM API (default: http://127.0.0.1:11435)
  - Tested with [llamafile](https://github.com/Mozilla-Ocho/llamafile), [ollama](https://ollama.com), [koboldcpp](https://github.com/LostRuins/koboldcpp), or any OpenAI-compatible proxy

## Quick Start

```bash
# Build etch-gen
cargo build --release

# Generate a blockchain from a prompt
./target/release/etch-gen generate \
  --prompt "A blockchain optimized for high throughput with fast block times and low scrypt difficulty for testing" \
  --output-dir ./generated \
  --api-url http://127.0.0.1:11435 \
  --model qwen2.5-coder-14b-instruct-q4-k-m

# The generated crate is ready to use
cd ./generated/<design_name>
cargo check
cargo run -- generate-key
```

## Using Presets (No LLM Required)

```bash
# Fast test configuration (low difficulty, fast blocks)
./target/release/etch-gen preset fast-test --output-dir ./generated

# Mainnet-like configuration
./target/release/etch-gen preset mainnet --output-dir ./generated
```

## CLI Reference

```
Usage: etch-gen <COMMAND>

Commands:
  generate  Generate a blockchain from a natural language prompt
  preset    Use a predefined configuration
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### generate

```
Usage: etch-gen generate [OPTIONS] --prompt <PROMPT>

Options:
  -p, --prompt <PROMPT>        Natural language prompt describing desired blockchain
  -o, --output-dir <OUTPUT_DIR>  Output directory [default: ./generated]
  -a, --api-url <API_URL>       LLM API base URL [default: http://127.0.0.1:11435]
  -m, --model <MODEL>           LLM model name [default: qwen2.5-coder-14b-instruct-q4-k-m]
      --benchmarks              Include benchmark module
  -h, --help                    Print help
```

### preset

```
Usage: etch-gen preset [OPTIONS] <PRESET>

Arguments:
  <PRESET>  [possible values: mainnet, fast-test]

Options:
  -o, --output-dir <OUTPUT_DIR>  [default: ./generated]
      --benchmarks               Include benchmark module
  -h, --help                     Print help
```

## Architecture

| Module | Responsibility |
|--------|---------------|
| `cli` | Clap argument parsing |
| `llm::client` | HTTP client to OpenAI-compatible API with retries |
| `llm::prompt` | System prompt template with JSON schema |
| `llm::parser` | Extract and parse JSON from LLM response |
| `design` | Blockchain design data structures and validation |
| `design::presets` | Offline fallback configurations |
| `factory::manifest` | Generate Cargo.toml for the output crate |
| `factory::renderer` | Render source templates with design parameters |
| `factory::writer` | Write generated files to disk |

## Design Parameters

The LLM returns a JSON object with these fields:

```json
{
  "design_name": "snake_case_identifier",
  "description": "human readable summary",
  "consensus": {
    "scrypt_n": 32768,
    "scrypt_r": 8,
    "scrypt_p": 1,
    "scrypt_len": 32,
    "target_block_time_secs": 120,
    "max_payloads_per_block": 1024,
    "payload_size_bytes": 256,
    "blocks_per_year": 262800,
    "difficulty_adjustment_period_blocks": 262800,
    "difficulty_vote_window_blocks": 1000,
    "block_reward": 500000000,
    "reward_decimals": 9,
    "max_supply": 21000000000000000,
    "prune_depth_blocks": 1000
  },
  "optimization": {
    "throughput_weight": 0.33,
    "latency_weight": 0.33,
    "disk_efficiency_weight": 0.34
  },
  "network": {
    "p2p_protocol_version": "0.1.0",
    "listen_port": 6262,
    "gossipsub_heartbeat_secs": 10
  }
}
```

## Testing

```bash
# Unit tests
cargo test --lib

# Integration tests (generates and compiles crates)
cargo test --test integration_test

# Parser tests
cargo test --test parser_test
```

## License

MIT
