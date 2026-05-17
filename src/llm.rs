use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::Config;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LlmConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

impl From<&Config> for LlmConfig {
    fn from(config: &Config) -> Self {
        Self {
            base_url: config.llm_base_url.clone(),
            api_key: config.llm_api_key.clone(),
            model: config.llm_model.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LlmClient {
    config: LlmConfig,
    http_client: Client,
}

impl LlmClient {
    pub fn new(config: LlmConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
    }

    pub fn config(&self) -> &LlmConfig {
        &self.config
    }

    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    pub async fn complete(
        &self,
        _request: CompletionRequest,
    ) -> Result<CompletionResponse, LlmError> {
        Err(LlmError::NotImplemented("LLM completion HTTP call"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub prompt: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub message: String,
}

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("{0} is not implemented yet")]
    NotImplemented(&'static str),

    #[error("LLM request failed")]
    Request(#[from] reqwest::Error),
}
