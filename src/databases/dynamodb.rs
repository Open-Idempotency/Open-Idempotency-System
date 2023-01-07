use std::sync::Arc;
use std::time::Duration;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use async_trait::async_trait;
use crate::databases::database::{DbConfig, IDatabase};

pub struct DynamodbClient {
    client: Client,
    table_name: String,
}

#[async_trait]
impl IDatabase for DynamodbClient {

    async fn exists(&self, key: String, app_id: String) -> bool {
        // let request = &self.client
        //     .get_item()
        //     .table_name(&self.table_name)
        //     .item(
        //         "key",
        //         AttributeValue::S(String::from(
        //             format!("{}:{}",app_id, key),
        //         )),
        //     ).send().await;
        return true
    }

    async fn delete (&self, key: String, app_id: String){
        // let request = client
        //     .delete_item()
        //     .table_name(tablename)
        //     .item(
        //         "key",
        //         AttributeValue::S(String::from(
        //             format!("{}:{}",self.project_name, key),
        //         )),
        //     ).send().await?;
    }

    async fn put (&self, key: String, app_id: String, ttl: Option<Duration>){
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
    }



}

impl DynamodbClient {
    pub async fn new (config: DbConfig) -> Arc<dyn IDatabase> {
        let shared_config = aws_config::load_from_env().await;
        return Arc::new(DynamodbClient {
            client: Client::new(&shared_config),
            table_name: config.table_name.unwrap()
        });
    }
}