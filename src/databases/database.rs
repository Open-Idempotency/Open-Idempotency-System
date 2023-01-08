use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct DbConfig {
    pub url: String,
    pub table_name: Option<String>,
    pub keyspace: Option<String>,
    pub ttl: Option<Duration>
}

#[async_trait]
pub trait IDatabase {
    async fn exists(&mut self, key: String, app_id: String) -> Result<MessageStatusDef, Box<dyn Error>>;
    async fn delete (&mut self, key: String, app_id: String) -> Result<(), Box<dyn Error>>;
    async fn put (&mut self, key: String, app_id: String, value: IdempotencyTransaction, ttl: Option<Duration>) -> Result<(), Box<dyn Error>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdempotencyTransaction {
    pub status: MessageStatusDef,
    pub response: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessageStatusDef {
    None = 0,
    InProgress = 1,
    Completed = 2,
}


pub fn combine_key(key: String, app_id: String) -> String {
    let mut full_key = app_id.clone();
    full_key.push_str(":");
    full_key.push_str(&key[..]);
    full_key
}

// async fn new (config: DbConfig) -> Box<dyn IDatabase>;