use crate::design::{BlockchainDesign, DesignError};
use serde_json;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("JSON parse failed: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Validation failed: {0}")]
    ValidationError(#[from] DesignError),
    #[error("No content in LLM response")]
    EmptyResponse,
    #[error("Failed to extract JSON from markdown: {0}")]
    MarkdownExtractionError(String),
}

pub struct DesignParser;

impl Default for DesignParser {
    fn default() -> Self {
        Self::new()
    }
}

impl DesignParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, content: &str) -> Result<BlockchainDesign, ParseError> {
        let json_str = Self::extract_json(content)?;
        let design: BlockchainDesign = serde_json::from_str(&json_str)?;
        design.validate()?;
        Ok(design)
    }

    fn extract_json(content: &str) -> Result<String, ParseError> {
        let trimmed = content.trim();

        // Find ```json or ``` block and extract content
        if let Some(start) = trimmed.find("```") {
            let after_fence = &trimmed[start + 3..];
            let code_start = if let Some(nl) = after_fence.find('\n') {
                nl + 1
            } else {
                0
            };
            let code = &after_fence[code_start..];
            if let Some(end) = code.find("```") {
                return Ok(code[..end].trim().to_string());
            }
        }

        Ok(trimmed.to_string())
    }
}
