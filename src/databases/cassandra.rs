use std::error::Error;
use std::sync::Arc;
// use cassandra_cpp::Cluster;
use std::time::Duration;
use async_trait::async_trait;
use crate::databases::database::{DbConfig, IDatabase, IdempotencyTransaction, MessageStatusDef};
pub struct CassandraClient {
    // client: Option<cassandra_cpp::Session>,
    table_name: String,
    config: DbConfig
}

#[async_trait]
impl IDatabase for CassandraClient {
    async fn exists(&mut self, key: String, app_id: String)  -> Result<MessageStatusDef, Box<dyn Error>>{
        Ok(MessageStatusDef::None)
    }
    async fn delete (&mut self, key: String, app_id: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    async fn put (&mut self, key: String, app_id: String, value: IdempotencyTransaction, ttl: Option<Duration>) -> Result<(), Box<dyn Error>>{
        Ok(())
    }

}

impl CassandraClient {
    pub fn new (config: DbConfig) -> Box<dyn IDatabase> {
        // let mut cluster = Cluster::default();
        // cluster.set_contact_points(config.url).unwrap();
        // was: cluster.connect_keyspace(config.keyspace).unwrap()
        let c = CassandraClient {
            // client: None,
            table_name: config.table_name.clone().unwrap(),
            config: config.clone()
        };
        return Box::new(c);
    }
}

