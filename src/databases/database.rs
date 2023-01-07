use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;

pub struct DbConfig {
    pub url: String,
    pub table_name: Option<String>,
    pub keyspace: Option<String>,
    pub ttl: Option<Duration>
}

#[async_trait]
pub trait IDatabase {
    async fn exists(&self, key: String, app_id: String) -> bool;
    async fn delete (&self, key: String, app_id: String);
    async fn put (&self, key: String, app_id: String, ttl: Option<Duration>);
}

// async fn new (config: DbConfig) -> Arc<dyn IDatabase>;