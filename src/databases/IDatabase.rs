use std::time::Duration;

struct DbConfig {
    table_name: Option<str>,
    url: str,
    keyspace: Option<str>,
    ttl: Option<Duration>
}

trait IDatabase {
    async fn exists(&self, key: uuid, app_id: String) -> bool;
    async fn delete (&self, key: uuid, app_id: String);
    async fn put (&self, key: uuid, app_id: String, ttl: Option<Duration>);
    async fn init (&self, config: DbConfig) -> dyn IDatabase;
}