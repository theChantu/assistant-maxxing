use thiserror::Error;
use tracing::info;

use crate::{
    config::Config,
    llm::{LlmClient, LlmConfig, LlmError},
    scheduler::Scheduler,
};

#[derive(Debug)]
pub struct BotApp {
    config: Config,
    llm_client: LlmClient,
    scheduler: Scheduler,
}

impl BotApp {
    pub fn new(config: Config) -> Self {
        let llm_client = LlmClient::new(LlmConfig::from(&config));

        Self {
            config,
            llm_client,
            scheduler: Scheduler::new(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn llm_client(&self) -> &LlmClient {
        &self.llm_client
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    pub async fn run(&self) -> Result<(), BotError> {
        info!("bot scaffold initialized");
        Err(BotError::NotImplemented("Telegram polling loop"))
    }
}

#[derive(Debug, Error)]
pub enum BotError {
    #[error("{0} is not implemented yet")]
    NotImplemented(&'static str),

    #[error(transparent)]
    Llm(#[from] LlmError),
}
