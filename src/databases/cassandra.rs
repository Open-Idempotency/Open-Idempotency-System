use std::error::Error;
use std::time::Duration;
use async_trait::async_trait;
use scylla::transport::session::{IntoTypedRows, Session};
use scylla::SessionBuilder;


use crate::databases::database::{DbConfig, IDatabase, IdempotencyTransaction};
use crate::databases::database::MessageStatusDef::Completed;


pub struct CassandraClient {
    client: Session,
    config: DbConfig
}


unsafe impl Send for CassandraClient {

}


#[async_trait]
impl IDatabase for CassandraClient {
    async fn exists(&mut self, key: String, app_id: String)  -> Result<IdempotencyTransaction, Box<dyn Error + Send + Sync>>{
        // Key should be a composite key of key and app_id
        let table = self.config.table_name.clone().unwrap();
        let keyspace  = self.config.keyspace.clone().unwrap();
        let query_string = format!("SELECT \"clientId\", \"appId\", \"value\" FROM \"{}\".\"{}\" WHERE \"clientId\" = '{}' AND \"appId\" = '{}';", keyspace, table, key, app_id);

        if let Some(rows) = self.client.query(query_string.to_string(), &[]).await?.rows {
            for row in rows.into_typed::<(String, String, String)>() {
                let (_, _, value) = row?;

                let deserialized= IdempotencyTransaction {
                    status: Completed,
                    response: value.to_string(),
                };
                return Ok(deserialized);
            }
        }
        return Ok(IdempotencyTransaction::new_default_none());
    }
    async fn delete (&mut self, key: String, app_id: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        let table = self.config.table_name.clone().unwrap();
        let keyspace  = self.config.keyspace.clone().unwrap();
        let query_string = format!("DELETE FROM \"{}\".\"{}\" WHERE \"clientId\" = '{}' AND \"appId\" = '{}';", keyspace, table, key, app_id);

        self.client.query(query_string, &[]).await?;


        Ok(())
    }
    async fn put (&mut self, key: String, app_id: String, value: IdempotencyTransaction, ttl: Option<Duration>) -> Result<(), Box<dyn Error + Send + Sync>>{
        let table = self.config.table_name.clone().unwrap();
        let keyspace  = self.config.keyspace.clone().unwrap();
        let mut time_in_seconds_string = (60 * 60 * 48).to_string();
        match ttl {
            Some(time) => { time_in_seconds_string = time.as_secs().to_string(); }
            None => {
                let config_ttl = self.config.ttl;
                match config_ttl {
                    Some(c_time) => { time_in_seconds_string = c_time.as_secs().to_string(); }
                    None => {}
                }
            }
        }
        let query_string = format!("INSERT INTO \"{}\".\"{}\" (\"clientId\", \"appId\", \"value\") VALUES ('{}', '{}' ,'{}') USING TTL {};", keyspace, table, key, app_id, value.response, time_in_seconds_string);

        self.client.query(query_string,&[]).await?;

        Ok(())
    }

}


impl CassandraClient {
    pub async fn new (config: DbConfig) -> Box<dyn IDatabase + Send> {
        let hostname = format!("{}",config.url.clone());
        let client_connection: Session = SessionBuilder::new().known_node(hostname.to_string()).build().await.unwrap();

        let c = CassandraClient {
            client: client_connection,
            config: config.clone()
        };
        return Box::new(c);
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::databases::cassandra::CassandraClient;
    use crate::databases::database::{DatabaseOption, DbConfig, IDatabase, IdempotencyTransaction, MessageStatusDef};

    async fn init_client() -> Box<dyn IDatabase> {
        let c = CassandraClient::new(
            DbConfig{
                table_name: Some(String::from("table_test")),
                url: String::from("127.0.0.1:9042"),
                keyspace: Some(String::from("keyspace_test")),
                ttl: Some(Duration::from_secs(30)),
                database_option: DatabaseOption::Cassandra
            }).await;
        c
    }


    #[tokio::test]
    pub async fn test_delete_cassandra() {
        let mut c =  init_client().await;

        c.put("cl01".to_string(), "ap02".to_string(), IdempotencyTransaction {
            response: String::from("Value"),
            status: MessageStatusDef::Completed
        },Some(Duration::from_secs(30))).await.unwrap();

        let result = c.exists("cl01".to_string(), "ap02".to_string()).await.unwrap();
        assert_eq!(result.status, MessageStatusDef::Completed);
        assert_eq!(result.response, "Value");

        c.delete("cl01".to_string(), "ap02".to_string()).await.unwrap();


        let result = c.exists("cl01".to_string(), "ap02".to_string()).await.unwrap();
        assert_eq!(result.status, MessageStatusDef::None);
    }

    #[tokio::test]
    pub async fn test_put_cassandra() {
        let mut c =  init_client().await;

        c.put("cl01".to_string(), "ap99".to_string(), IdempotencyTransaction {
            response: String::from("Bucket"),
            status: MessageStatusDef::Completed
        },Some(Duration::from_secs(3000))).await.unwrap();

        let result = c.exists("cl01".to_string(), "ap99".to_string()).await.unwrap();
        assert_eq!(result.status, MessageStatusDef::Completed);
        assert_eq!(result.response, "Bucket");

        c.delete("cl01".to_string(), "ap99".to_string()).await.unwrap();
    }
}