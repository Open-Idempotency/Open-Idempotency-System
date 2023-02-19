use std::error::Error;
use std::time::Duration;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::open_idempotency::MessageStatus;

#[derive(Clone)]
pub struct DbConfig {
    pub url: String,
    pub table_name: Option<String>,
    pub keyspace: Option<String>,
    pub ttl: Option<Duration>,
    pub database_option: DatabaseOption
}

impl DbConfig {
    pub fn resolve_ttl(&self, ttl: &Option<Duration>) -> usize {
        usize::try_from(
            (match ttl {
                Some(v) => v.clone(),
                None => self.ttl.clone().unwrap()
            }).clone().as_secs()
        ).unwrap()
    }
}

#[derive(Clone)]
pub enum DatabaseOption {
    Redis,
    Dynamo,
    Cassandra
}

#[async_trait]
pub trait IDatabase {
    async fn exists(&mut self, key: String, app_id: String) -> Result<IdempotencyTransaction, Box<dyn Error + Send + Sync>>;
    async fn delete (&mut self, key: String, app_id: String) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn insert (&mut self, key: String, app_id: String, value: IdempotencyTransaction, ttl: Option<Duration>) -> Result<(), Box<dyn Error + Send + Sync>>;
    // does not reset ttl
    async fn update (&mut self, key: String, app_id: String, value: IdempotencyTransaction) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdempotencyTransaction {
    pub status: MessageStatusDef,
    pub stage: String,
    pub response: String
}

impl IdempotencyTransaction {
    pub fn new_from_status(status: MessageStatusDef) -> IdempotencyTransaction {
        IdempotencyTransaction {
            status,
            stage: String::from(""),
            response: String::from("")
        }
    }

    pub fn new_default_none() -> IdempotencyTransaction {
        IdempotencyTransaction {
            status: MessageStatusDef::None,
            stage: String::from(""),
            response: String::from("")
        }
    }

    pub fn new_default_in_progress() -> IdempotencyTransaction {
        IdempotencyTransaction {
            status: MessageStatusDef::InProgress,
            stage: String::from(""),
            response: String::from("")
        }
    }
    pub fn new(status: MessageStatusDef, response: String) -> IdempotencyTransaction {
        IdempotencyTransaction {
            status,
            stage: String::from(""),
            response
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessageStatusDef {
    None = 0,
    InProgress = 1,
    Completed = 2,
    Failed = 3,
}
impl MessageStatusDef {
    pub fn map_to_grpc(&self) -> MessageStatus {
        match self {
            MessageStatusDef::None => { MessageStatus::None },
            MessageStatusDef::Completed => { MessageStatus::Completed },
            MessageStatusDef::InProgress => { MessageStatus::InProgress }
            MessageStatusDef::Failed => { MessageStatus::Failed }
        }
    }
}



pub fn combine_key(key: String, app_id: String) -> String {
    let mut full_key = app_id.clone();
    full_key.push_str(":");
    full_key.push_str(&key[..]);
    full_key
}

// async fn new (config: DbConfig) -> Box<dyn IDatabase>;