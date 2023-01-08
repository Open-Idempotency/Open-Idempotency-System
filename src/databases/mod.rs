mod redis_driver;
mod dynamodb;
mod cassandra;
pub mod database;
use std::sync::Arc;
use database::IDatabase;
use redis_driver::RedisClient;
use crate::databases::cassandra::CassandraClient;
use crate::databases::database::{DatabaseOption, DbConfig};
use crate::databases::dynamodb::DynamodbClient;

fn get_config() -> DbConfig{
     DbConfig {
        url:  String::from(""),
        table_name: None,
        keyspace: None,
        ttl: None,
         database_option: DatabaseOption::Redis
    }
}

pub async fn create_database() -> Box<dyn IDatabase> {
    let config = get_config();
    return match config.database_option {
        DatabaseOption::Redis => {
            RedisClient::new(config.clone())
        },
        DatabaseOption::Cassandra => {
            CassandraClient::new(config.clone())
        },
        DatabaseOption::Dynamo => {
            DynamodbClient::new(config.clone()).await
        }
    }
}
