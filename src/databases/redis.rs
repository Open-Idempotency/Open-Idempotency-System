extern crate redis;

use std::sync::Arc;
use std::time::Duration;
use redis::{Client, Commands, Connection, RedisResult};
use crate::databases::database::{IDatabase, DbConfig};

pub struct RedisClient{
    client: Client
}

impl IDatabase for RedisClient {

    fn exists(&self, key: String, app_id: String) -> bool{
        let conn = &mut (self.conn).unwrap();
        conn.get(format!("{}:{}",self.project_name, key))?;
        return true;
    }

    fn delete (&self, key: String, app_id: String){
        let conn = &mut (self.conn).unwrap();
        let ret: () = conn.del(key)?;
    }

    fn put (&self, key: String, app_id: String, ttl: Duration) {
        let conn = &mut (self.conn).unwrap();
        let ret : () =  conn.set_ex(format!("{}:{}",self.project_name, key),"",ttl.as_secs().to_usize())?;
    }

}

impl RedisClient {
    pub(crate) fn new (config: DbConfig) -> Arc<dyn IDatabase> {
        let r = RedisClient {
            client: Client::open(config.url).unwrap()
        };
        return Arc::new(r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test!]
    pub fn test_connect() {
        let c = RedisClient::new(DbConfig{
            table_name: None,
            url: String::from("redis://default:redispw@localhost:49153"),
            keyspace: None,
            ttl: None,
        });
        c.put("mykey", String::from("my_app"), Some(Duration::from_secs(30)));
    }
}