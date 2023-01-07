mod redis_driver;
// mod dynamodb;
// mod cassandra;
pub mod database;
use std::sync::Arc;
use database::IDatabase;
use redis_driver::RedisClient;
use crate::databases::database::DbConfig;

pub fn create_database() -> Arc<dyn IDatabase> {
    let config = DbConfig {
        url:  String::from(""),
        table_name: None,
        keyspace: None,
        ttl: None,
    };
    let  redis_db = RedisClient::new(config);
    //
    // let casandra =
    redis_db
}
