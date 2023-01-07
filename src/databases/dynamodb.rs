use std::time::Duration;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;

pub struct DynamodbClient {
    client: aws_sdk_dynamodb::client,
    table_name: str,
}

impl IDatabase for DynamodbClient {

    async fn exists(&self, key: uuid, app_id: String) -> bool {
        let request = client
            .get_item()
            .table_name(tablename)
            .item(
                "key",
                AttributeValue::S(String::from(
                    format!("{}:{}",self.project_name, key),
                )),
            ).send().await?;
        return true
    }

    async fn delete (&self, key: uuid, app_id: String){
        let request = client
            .delete_item()
            .table_name(tablename)
            .item(
                "key",
                AttributeValue::S(String::from(
                    format!("{}:{}",self.project_name, key),
                )),
            ).send().await?;
    }

    async fn put (&self, key: uuid, app_id: String, ttl: Duration){
        let request = client
            .put_item()
            .table_name(tablename)
            .item(
                "key",
                AttributeValue::S(String::from(
                    format!("{}:{}",self.project_name, key),
                )),
            ).item(
                "ttl",
                AttributeValue::N(ttl.seconds),
            ).send().await?;
    }

    async fn init (&mut self, config: DbConfig) -> dyn IDatabase{
        let shared_config = aws_config::load_from_env().await;
        self.client = Client::new(&shared_config);
        self.table_name = config.table_name;
        return self;
    }

}