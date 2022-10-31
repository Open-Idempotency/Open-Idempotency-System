struct DbConfig {
    project_name: str,
    table_name: Option<str>,
    url: Option<str>,
}

trait IDatabase {
    async fn exists(&self, key: uuid) -> bool;
    async fn delete (&self, key: uuid);
    async fn put (&self, key: uuid, ttl: prost_types::Timestamp);
    async fn init (&self, config: DbConfig) -> dyn IDatabase;
}