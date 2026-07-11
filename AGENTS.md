# PROJECT KNOWLEDGE BASE

**Generated:** 2026-05-24
**Commit:** bcaa15c
**Branch:** main

## OVERVIEW

AI-powered blockchain generator using the Etch architecture. A Rust CLI tool that takes natural language prompts, consults a local OpenAI-compatible LLM, and emits complete standalone Rust blockchain crates that compile with `cargo check`.

## STRUCTURE

```
.
├── src/
│   ├── main.rs          # CLI entry point (tokio async)
│   ├── lib.rs           # Module re-exports
│   ├── cli.rs           # Clap argument definitions
│   ├── design/          # Blockchain parameter data structures + validation
│   ├── factory/         # Code generation: manifest, renderer, writer
│   ├── llm/             # HTTP client + JSON parsing for LLM API
│   └── templates/       # Handlebars templates for generated crates
├── tests/               # Integration tests (compile generated crates)
└── generated/           # Output directory for generated blockchain crates
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add CLI args | `src/cli.rs` | Clap derive macros |
| Change LLM API behavior | `src/llm/` | Retries, parsing, prompts |
| Modify blockchain params | `src/design/mod.rs` | Structs + validation rules |
| Add generated crate files | `src/templates/*.hbs` | Handlebars-ish templates |
| Change output Cargo.toml | `src/factory/manifest.rs` | Hardcoded dep versions |
| Integration test | `tests/integration_test.rs` | Compiles generated crates |

## CODE MAP

| Symbol | Type | Location | Role |
|--------|------|----------|------|
| `Cli` | struct | `src/cli.rs` | Top-level clap parser |
| `Commands` | enum | `src/cli.rs` | Generate / Preset subcommands |
| `LlmClient` | struct | `src/llm/client.rs` | HTTP client with retries |
| `DesignParser` | struct | `src/llm/parser.rs` | JSON extraction + validation |
| `BlockchainDesign` | struct | `src/design/mod.rs` | Root design data structure |
| `ConsensusParameters` | struct | `src/design/mod.rs` | Scrypt + block params |
| `ManifestGenerator` | struct | `src/factory/manifest.rs` | Generates output Cargo.toml |
| `TemplateRenderer` | struct | `src/factory/renderer.rs` | Renders templates with design params |
| `CrateWriter` | struct | `src/factory/writer.rs` | Writes files to disk |

## CONVENTIONS

- Rust 2021 edition, minimum version 1.75
- No custom rustfmt/clippy config (defaults)
- `thiserror` for error types
- `tracing` for logging (subscriber initialized in `main`)

## ANTI-PATTERNS (THIS PROJECT)

- `PromptTemplate::render()` is a pass-through (no actual templating logic)
- `TemplateRenderer` uses simple string replacement, not real Handlebars (despite `handlebars` dep in Cargo.toml)
- `Cargo.lock` is in `.gitignore` but appears committed (contradiction)
- No `Authorization` header commented out in LLM client (line 95)

## COMMANDS

```bash
# Build
cargo build --release

# Run with LLM
./target/release/etch-gen generate \
  --prompt "fast block times for testing" \
  --output-dir ./generated \
  --api-url http://127.0.0.1:11435

# Run with preset (no LLM)
./target/release/etch-gen preset fast-test --output-dir ./generated

# Test
cargo test --lib
cargo test --test integration_test
cargo test --test parser_test
```

## NOTES

- Default LLM endpoint: `http://127.0.0.1:11435` (Ollama/llamafile/koboldcpp)
- Default model: `qwen2.5-coder-14b-instruct-q4-k-m`
- Generated crates depend on `libp2p`, `scrypt`, `ed25519-dalek`, `postcard`
- Integration tests actually compile generated crates via `cargo check`
- No CI/CD at all; no Makefile, justfile, or build.rs
- All tests are integration tests (zero inline `#[cfg(test)]` in src/)
- `tests/integration_test.rs` hardcodes `.current_dir("/var/home/a/code/etch-gen")`

## Deployment

No Dagger module or recognized deployment configuration was found.

General redeploy process:

1. Commit and push changes to the default branch.
2. Trigger the relevant CI/CD pipeline or run the documented deploy command.
3. If the project is served via GitHub Pages, the site redeploys automatically after the push.
