use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use async_trait::async_trait;
use crate::databases::database::{DbConfig, IDatabase, IdempotencyTransaction, MessageStatusDef};

pub struct DynamodbClient {
    client: Client,
    table_name: String,
    config: DbConfig
}

#[async_trait]
impl IDatabase for DynamodbClient {

    async fn exists(&mut self, key: String, app_id: String)  -> Result<IdempotencyTransaction, Box<dyn Error>> {
        // let request = &self.client
        //     .get_item()
        //     .table_name(&self.table_name)
        //     .item(
        //         "key",
        //         AttributeValue::S(String::from(
        //             format!("{}:{}",app_id, key),
        //         )),
        //     ).send().await;
        Ok(IdempotencyTransaction::new_default_none())
    }

    async fn delete (&mut self, key: String, app_id: String) -> Result<(), Box<dyn Error>> {
        // let request = client
        //     .delete_item()
        //     .table_name(tablename)
        //     .item(
        //         "key",
        //         AttributeValue::S(String::from(
        //             format!("{}:{}",self.project_name, key),
        //         )),
        //     ).send().await?;
        Ok(())
    }

    async fn put (&mut self, key: String, app_id: String, value: IdempotencyTransaction, ttl: Option<Duration>) -> Result<(), Box<dyn Error>> {
        // let request = client
        //     .put_item()
        //     .table_name(tablename)
        //     .item(
        //         "key",
        //         AttributeValue::S(String::from(
        //             format!("{}:{}",self.project_name, key),
        //         )),
        //     ).item(
        //         "ttl",
        //         AttributeValue::N(ttl.seconds),
        //     ).send().await?;
        Ok(())
    }
}

impl DynamodbClient {
    pub async fn new (config: DbConfig) -> Box<dyn IDatabase> {
        let shared_config = aws_config::load_from_env().await;
        return Box::new(DynamodbClient {
            client: Client::new(&shared_config),
            table_name: config.table_name.clone().unwrap(),
            config: config.clone()
        });
    }
}