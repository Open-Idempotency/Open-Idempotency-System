struct DynamodbClient {
    client: aws_sdk_dynamodb::client,
    project_name: str,
    table_name: str,
}

impl IDatabase for DynamodbClient {

    async fn exists(&self, key: uuid) -> bool {
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

    async fn delete (&self, key: uuid){
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

    async fn put (&self, key: uuid, ttl: prost_types::Timestamp){
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

    async fn init (&self, config: DbConfig) -> dyn IDatabase{
        let shared_config = aws_config::load_from_env().await;
        self.client = Client::new(&shared_config);
        self.project_name = config.project_name;
        self.table_name = config.table_name;
        return self;
    }

}