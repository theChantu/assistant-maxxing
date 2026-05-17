use std::{env, fmt};

use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub telegram_bot_token: String,
    pub llm_base_url: String,
    pub llm_api_key: String,
    pub llm_model: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| env::var(key).ok())
    }

    pub fn from_lookup(
        mut lookup: impl FnMut(&str) -> Option<String>,
    ) -> Result<Self, ConfigError> {
        Ok(Self {
            telegram_bot_token: required_value(&mut lookup, ConfigKey::TelegramBotToken)?,
            llm_base_url: required_value(&mut lookup, ConfigKey::LlmBaseUrl)?,
            llm_api_key: required_value(&mut lookup, ConfigKey::LlmApiKey)?,
            llm_model: required_value(&mut lookup, ConfigKey::LlmModel)?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigKey {
    TelegramBotToken,
    LlmBaseUrl,
    LlmApiKey,
    LlmModel,
}

impl ConfigKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TelegramBotToken => "TELEGRAM_BOT_TOKEN",
            Self::LlmBaseUrl => "LLM_BASE_URL",
            Self::LlmApiKey => "LLM_API_KEY",
            Self::LlmModel => "LLM_MODEL",
        }
    }
}

impl fmt::Display for ConfigKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConfigError {
    #[error("missing required environment variable {0}")]
    Missing(ConfigKey),
}

fn required_value(
    lookup: &mut impl FnMut(&str) -> Option<String>,
    key: ConfigKey,
) -> Result<String, ConfigError> {
    lookup(key.as_str())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or(ConfigError::Missing(key))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Config, ConfigError, ConfigKey};

    #[test]
    fn loads_required_values() {
        let values = HashMap::from([
            ("TELEGRAM_BOT_TOKEN", "telegram-token"),
            ("LLM_BASE_URL", "http://localhost:8080/v1"),
            ("LLM_API_KEY", "llm-key"),
            ("LLM_MODEL", "local-model"),
        ]);

        let config = Config::from_lookup(|key| values.get(key).map(|value| value.to_string()))
            .expect("config should load");

        assert_eq!(config.telegram_bot_token, "telegram-token");
        assert_eq!(config.llm_base_url, "http://localhost:8080/v1");
        assert_eq!(config.llm_api_key, "llm-key");
        assert_eq!(config.llm_model, "local-model");
    }

    #[test]
    fn rejects_missing_values() {
        let error = Config::from_lookup(|_| None).expect_err("config should fail");

        assert_eq!(error, ConfigError::Missing(ConfigKey::TelegramBotToken));
    }

    #[test]
    fn rejects_blank_values() {
        let values = HashMap::from([("TELEGRAM_BOT_TOKEN", "   ")]);

        let error = Config::from_lookup(|key| values.get(key).map(|value| value.to_string()))
            .expect_err("config should fail");

        assert_eq!(error, ConfigError::Missing(ConfigKey::TelegramBotToken));
    }
}
