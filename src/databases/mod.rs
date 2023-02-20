mod redis_driver;
mod dynamodb;
mod cassandra;
pub mod database;

use tokio::sync::Mutex;
use std::sync::{Arc};
use database::IDatabase;
use redis_driver::RedisClient;
use crate::databases::cassandra::CassandraClient;
use crate::databases::database::{DatabaseOption, DbConfig};
use crate::databases::dynamodb::DynamodbClient;
use tokio::runtime::Runtime;

fn get_config() -> DbConfig{
    DbConfig {
        url:  String::from("redis://default:redispw@localhost:49153"),
        table_name: None,
        keyspace: None,
        ttl: None,
        database_option: DatabaseOption::Redis
    }
}

pub fn create_database_mutex_sync() -> Arc<Mutex<Box<dyn IDatabase + Send>>> {
    let v = Runtime::new().unwrap().block_on(create_database_mutex());
    Arc::new(v)
}
pub async fn create_database_mutex() -> Mutex<Box<dyn IDatabase + Send>> {
    Mutex::new(create_database().await)
}
pub async fn create_database() -> Box<dyn IDatabase + Send> {
    let config = get_config();
    return match config.database_option {
        DatabaseOption::Redis => {
            RedisClient::new(config.clone()).await
        },
        DatabaseOption::Cassandra => {
            CassandraClient::new(config.clone()).await
        },
        DatabaseOption::Dynamo => {
            DynamodbClient::new(config.clone()).await
        }
    }
}


