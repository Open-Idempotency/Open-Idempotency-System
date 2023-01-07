extern crate redis;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use redis::{Client, Commands, Connection, RedisResult};
use crate::databases::database::{IDatabase, DbConfig, combine_key};
use std::convert::TryFrom;

pub struct RedisClient{
    client: Client,
    con: Connection,
    config: DbConfig
}


fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _ : () = con.set("my_key", 42).unwrap();
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

#[async_trait]
impl IDatabase for RedisClient {

    async fn exists(&mut self, key: String, app_id: String) -> bool{
        let val: bool = self.con.exists(combine_key(key, app_id), ).unwrap();
        return val;
    }

    async fn delete (&mut self, key: String, app_id: String){
        // let conn = &mut (self.conn).unwrap();
        // let ret: () = conn.del(key)?;
    }

    async fn put (&mut self, key: String, app_id: String, ttl: Option<Duration>) {
        let ttl_usize = usize::try_from(ttl.unwrap().as_secs()).unwrap();
        let _ : () = self.con.set_ex(combine_key(key, app_id), 42, ttl_usize).unwrap();
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

    #[tokio::test]
    pub async fn test_connect() {
        let mut c = RedisClient::new(DbConfig{
            table_name: None,
            url: String::from("redis://default:redispw@localhost:49153"),
            keyspace: None,
            ttl: None,
        });
        c.put(String::from("mykey"), String::from("my_app"), Some(Duration::from_secs(30))).await;

        let result = c.exists(String::from("mykey"), String::from("my_app")).await;
        println!("{:?}", result);
        assert_eq!(result, true);
    }

}