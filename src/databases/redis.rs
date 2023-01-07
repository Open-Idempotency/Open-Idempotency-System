extern crate redis;
use std::time::Duration;
use redis::{Client, Commands, Connection, RedisResult};

pub struct RedisClient{
    client:  redis::Client
}

impl IDatabase for RedisClient {

    async fn exists(&self, key: uuid, app_id: String) -> bool{
        let conn = &mut (self.conn).unwrap();
        conn.get(format!("{}:{}",self.project_name, key))?;
        return true;
    }

    async fn delete (&self, key: uuid, app_id: String){
        let conn = &mut (self.conn).unwrap();
        let ret: () = conn.del(key)?;
    }
    async fn put (&self, key: uuid, app_id: String, ttl: Duration){
        let conn = &mut (self.conn).unwrap();
        let ret : () =  conn.set_ex(format!("{}:{}",self.project_name, key),"",ttl)?;
    }

    async fn init (&mut self, config: DbConfig) -> dyn IDatabase{
        self.client = redis::Client::open(config.url).unwrap();
        return self;
    }
}