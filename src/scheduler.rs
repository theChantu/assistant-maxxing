use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: String,
    pub prompt: String,
    pub run_at_unix_timestamp: i64,
}

#[derive(Clone, Debug, Default)]
pub struct Scheduler;

impl Scheduler {
    pub fn new() -> Self {
        Self
    }

    pub async fn schedule(&self, _task: ScheduledTask) -> Result<(), SchedulerError> {
        Err(SchedulerError::NotImplemented("task scheduling"))
    }

    pub async fn run_pending(&self) -> Result<(), SchedulerError> {
        Err(SchedulerError::NotImplemented("pending task execution"))
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SchedulerError {
    #[error("{0} is not implemented yet")]
    NotImplemented(&'static str),
}
