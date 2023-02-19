use std::collections::HashMap;
use std::error::Error;

use std::hash::Hash;

use std::time::Duration;
use aws_config::endpoint::Endpoint;
use aws_config::meta::region::{ProvideRegion, RegionProviderChain};

use aws_sdk_dynamodb::Client;
use crate::databases::database::{DbConfig, IDatabase, IdempotencyTransaction, MessageStatusDef};
use aws_sdk_dynamodb::model::{AttributeDefinition, AttributeValue, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType};
use aws_types::region::Region;
use tonic::async_trait;
//  use testcontainers::*;
pub struct DynamodbClient {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
    config: DbConfig
}


unsafe impl Send for DynamodbClient {  // TODO comment me
}
pub struct Item {
    key : String, app_id: String, // composite key
    ttl: String,

}

#[async_trait]
impl IDatabase for DynamodbClient {

    async fn exists(&mut self, key: String, app_id: String) -> Result<IdempotencyTransaction, Box<dyn Error + Send + Sync>> {
        self.client
            .get_item()
            .table_name(&self.table_name)
            .key(
                "key",
                AttributeValue::S(String::from(
                    format!("{}:{}", app_id, key),
                )),
            ).send().await?;
        Ok(IdempotencyTransaction::new_default_none())
    }

    async fn delete(&mut self, key: String, app_id: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let request = self.client
            .delete_item()
            .table_name(&self.table_name)
            .key(
                "key",
                AttributeValue::S(String::from(
                    format!("{}:{}", app_id, key),
                )),
            ).send().await?;
        Ok(())
    }

    // TODO put function isn't working because item needs to be inserted as a composite key
    // currently rust is only seeing one key inserted
    async fn put(&mut self, key: String, app_id: String, value: IdempotencyTransaction, ttl: Option<Duration>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let composite_key = (app_id, key);
        let request = &self.client;

        let mut map: HashMap<String, AttributeValue> = HashMap::new();
        //map.insert(key.to_owned() AttributeValue::S()) ;
        match ttl {
            None => {
                request.put_item()
                    .table_name(&self.table_name)
                    .item(
                        "appid",
                        AttributeValue::S(composite_key.0)
                    )
                    .item(
                        "key",
                        AttributeValue::S(composite_key.1))
                    .send().await.unwrap();
            }
            Some(_) => {
                let duration = ttl.unwrap();
                request.put_item()
                    .table_name(&self.table_name)
                    .item(
                        "appid",
                        AttributeValue::S(composite_key.0), )
                    .item(
                        "key",
                        AttributeValue::S(composite_key.1)
                    )
                    .item(
                        "ttl",
                        AttributeValue::N(duration.as_secs().to_string()),
                    ).send().await.unwrap();
            }
        }
        Ok(())
    }
}
//} // TODO comment me

impl DynamodbClient {
    pub async fn new(config: DbConfig) -> Box<dyn IDatabase + Send> {
        //let mut shared_config = aws_config::load_from_env().await;
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");

        let actual_config = aws_config::from_env()
            .region(region_provider)
            .endpoint_resolver(Endpoint::immutable(&config.url).unwrap())
            .load()
            .await;

        let cli = Client::new(&actual_config);
        return Box::new(DynamodbClient {
            client: Client::new(&actual_config),
            table_name: config.table_name.clone().unwrap(),
            config: config.clone()
        });
    }
}

#[cfg(test)]
mod tests{
    use aws_config::endpoint::Endpoint;
    use aws_config::meta::region::RegionProviderChain;
    use aws_sdk_dynamodb::{Credentials};

    use crate::databases::database::DatabaseOption;
    use super::*;

    async fn init_client() -> Box<dyn IDatabase> {
        let c = DynamodbClient::new(DbConfig{
            table_name: Option::from("persistence".to_string()),
            url: String::from("http://localhost:4566"),
            keyspace: None,
            ttl: None,
            database_option: DatabaseOption::Dynamo
        });

        c.await
    }

    #[tokio::test]
    async fn test_put() {
        let mut c = init_client().await;
        let app_id = "thisApp".to_string();
        let key_id = "item1".to_string();
        let idem_trans = IdempotencyTransaction{ status: MessageStatusDef::None, response: "".to_string(), };
        let dur = Duration::new(60, 0);
        let ttl = dur;


        let r = c.put(key_id, app_id, idem_trans, Some(ttl));
        r.await.unwrap_err();
    }

    #[tokio::test]
    async fn dynamodb_local_create_table() {
        let _ = pretty_env_logger::try_init();

        /** this is provided by the testcontainer.rs dependency but the container it creates
               seems to not have its  8000 port exposed, pivoted to setting up local
               using localstack docker-compose file instead
               let node = docker.run(image);
               node.get_host_port_ipv4(52991);
         */

        let host_port = 4566;

        let table_name = "persistence".to_string();

        let ks1 = KeySchemaElement::builder()
            .attribute_name("appid".to_string())
            .key_type(KeyType::Hash)
            .build();

        let ks2 = KeySchemaElement::builder()
            .attribute_name("key".to_string())
            .key_type(KeyType::Range)
            .build();

        let provisioned_throughput = ProvisionedThroughput::builder()
            .read_capacity_units(10)
            .write_capacity_units(5)
            .build();

        let dynamodb = build_dynamodb_client(host_port).await;

        let _create_table_result = dynamodb
            .create_table()
            .table_name(table_name)
            .key_schema(ks1)
            .key_schema(ks2)
            .provisioned_throughput(provisioned_throughput)
            .send()
            .await.unwrap_err();

        let req = dynamodb.list_tables().limit(15);

        let list_tables_result = req.send().await.unwrap();// _or(not_found);

        assert_eq!(list_tables_result.table_names().unwrap().len(), 1);
    }


    async fn build_dynamodb_client(host_port: u16) -> Client {
        let endpoint_uri = format!("http://localhost:{}", host_port);

        let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
        let creds = Credentials::new("fakeKey", "fakeSecret", None, None, "test");

        let shared_config = aws_config::from_env()
            .region(region_provider)
            .endpoint_resolver(Endpoint::immutable(endpoint_uri).unwrap())
            .credentials_provider(creds)
            .load()
            .await;

        Client::new(&shared_config)
    }
}