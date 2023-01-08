extern crate redis;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use redis::{Client, Commands, Connection, RedisResult};
use crate::databases::database::{IDatabase, DbConfig, combine_key, MessageStatusDef, IdempotencyTransaction};
use std::convert::TryFrom;
use std::error::Error;

pub struct RedisClient{
    client: Client,
    con: Connection,
    config: DbConfig
}

#[async_trait]
impl IDatabase for RedisClient {

    async fn exists(&mut self, key: String, app_id: String) -> Result<MessageStatusDef, Box<dyn Error>> {
        let full_key = combine_key(key, app_id);
        let exists: bool = self.con.exists(&full_key, )?;
        if !exists {
            return Ok(MessageStatusDef::None);
        }
        // todo: get actual status
        let valString : String = self.con.get(&full_key)?;
        let deserialized: IdempotencyTransaction = serde_json::from_str(&valString).unwrap();
        return Ok(deserialized.status);
    }

    async fn delete (&mut self, key: String, app_id: String) -> Result<(), Box<dyn Error>> {
        self.con.del(combine_key(key, app_id))?;
        Ok(())
    }

    async fn put (&mut self, key: String, app_id: String, value: IdempotencyTransaction, ttl: Option<Duration>) -> Result<(), Box<dyn Error>>{
        let ttl_usize = usize::try_from(ttl.unwrap().as_secs()).unwrap();
        let _ : () = self.con.set_ex(combine_key(key, app_id), serde_json::to_string(&value).unwrap(), ttl_usize)?;
        Ok(())
    }

}

impl RedisClient {
    pub(crate) fn new (config: DbConfig) -> Box<dyn IDatabase> {
        let client = Client::open(config.url.clone()).unwrap();
        let con =  client.get_connection().unwrap();
        let r = RedisClient {
            client,
            con,
            config: config.clone()
        };
        return Box::new(r);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_client() -> Box<dyn IDatabase> {
        let c = RedisClient::new(DbConfig{
            table_name: None,
            url: String::from("redis://default:redispw@localhost:49153"),
            keyspace: None,
            ttl: None,
        });
        c
    }

    #[tokio::test]
    pub async fn test_put() {
        let mut c = init_client();
        c.put(String::from("mykey"), String::from("my_app"), IdempotencyTransaction {
            response: String::from("SomeString"),
            status: MessageStatusDef::Completed
        },Some(Duration::from_secs(30))).await.unwrap();
        let result = c.exists(String::from("mykey"), String::from("my_app")).await.unwrap();
        assert_eq!(result, MessageStatusDef::Completed);
    }


}