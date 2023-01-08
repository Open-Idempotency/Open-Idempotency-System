use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;

#[derive(Clone)]
pub struct DbConfig {
    pub url: String,
    pub table_name: Option<String>,
    pub keyspace: Option<String>,
    pub ttl: Option<Duration>
}

#[async_trait]
pub trait IDatabase {
    async fn exists(&mut self, key: String, app_id: String) -> bool;
    async fn delete (&mut self, key: String, app_id: String);
    async fn put (&mut self, key: String, app_id: String, ttl: Option<Duration>);
}

pub fn combine_key(key: String, app_id: String) -> String {
    let mut full_key = app_id.clone();
    full_key.push_str(":");
    full_key.push_str(&key[..]);
    full_key
}

// async fn new (config: DbConfig) -> Box<dyn IDatabase>;