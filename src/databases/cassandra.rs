use std::error::Error;
// use cassandra_cpp::Cluster;
use std::time::Duration;
use async_trait::async_trait;
use crate::databases::database::{DbConfig, IDatabase, IdempotencyTransaction, MessageStatusDef};
pub struct CassandraClient {
    // client: Option<cassandra_cpp::Session>,
    table_name: String,
    config: DbConfig
}
unsafe impl Send for CassandraClient {

}

#[async_trait]
impl IDatabase for CassandraClient {
    async fn exists(&mut self, key: String, app_id: String)  -> Result<IdempotencyTransaction, Box<dyn Error + Send + Sync>>{
        Ok(IdempotencyTransaction::new_default_none())
    }
    async fn delete (&mut self, key: String, app_id: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
    async fn put (&mut self, key: String, app_id: String, value: IdempotencyTransaction, ttl: Option<Duration>) -> Result<(), Box<dyn Error + Send + Sync>>{
        Ok(())
    }

}

impl CassandraClient {
    pub fn new (config: DbConfig) -> Box<dyn IDatabase + Send> {
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

