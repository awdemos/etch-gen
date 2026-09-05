use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, warn};

const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY_MS: u64 = 1000;

#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("API returned error: {0}")]
    ApiError(String),
    #[error("Max retries exceeded")]
    MaxRetries,
    #[error("No content in LLM response")]
    EmptyResponse,
}

#[derive(Clone)]
pub struct LlmClient {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
    stream: bool,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

impl LlmClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn chat_completion(&self, prompt: &str, model: &str) -> Result<String, LlmError> {
        let system_prompt = self.system_prompt();
        let request = ChatRequest {
            model: model.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            temperature: 0.2,
            max_tokens: 2048,
            stream: false,
        };

        let url = format!("{}/v1/chat/completions", self.base_url);
        let mut last_error = None;

        for attempt in 0..MAX_RETRIES {
            debug!("LLM request attempt {}/{}", attempt + 1, MAX_RETRIES);

            let response = self
                .client
                .post(&url)
                .header("Content-Type", "application/json")
                // .header("Authorization", format!("Bearer {}", api_key))
                .json(&request)
                .timeout(Duration::from_secs(120))
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let chat_resp: ChatResponse =
                            resp.json().await.map_err(LlmError::HttpError)?;
                        if let Some(choice) = chat_resp.choices.first() {
                            info!("LLM response received");
                            return Ok(choice.message.content.trim().to_string());
                        }
                        return Err(LlmError::EmptyResponse);
                    } else {
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        warn!("LLM API error (status {}): {}", status, text);
                        last_error = Some(LlmError::ApiError(format!("{}: {}", status, text)));
                    }
                }
                Err(e) => {
                    warn!("LLM request failed: {}", e);
                    last_error = Some(LlmError::HttpError(e));
                }
            }

            if attempt < MAX_RETRIES - 1 {
                let delay = INITIAL_RETRY_DELAY_MS * 2_u64.pow(attempt);
                info!("Retrying in {}ms...", delay);
                sleep(Duration::from_millis(delay)).await;
            }
        }

        Err(last_error.unwrap_or(LlmError::MaxRetries))
    }

    fn system_prompt(&self) -> String {
        r#"You are a blockchain systems designer. Given user requirements, output ONLY a JSON object matching this exact schema:

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

RULES:
- design_name must be snake_case (no spaces, no hyphens)
- scrypt_n must be a power of 2 between 1024 and 1048576
- target_block_time_secs must be between 10 and 3600
- max_payloads_per_block must be between 1 and 65536
- payload_size_bytes must be between 16 and 4096
- optimization weights must sum to 1.0
- Respond with ONLY the JSON object, no markdown, no explanations, no code blocks"#.to_string()
    }
}
