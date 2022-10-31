extern crate redis;

use redis::{Commands, Connection, RedisResult};

struct RedisClient{
    client:  redis::RedisResult<Connection>,
    project_name: str,
}

impl IDatabase for RedisClient {

    async fn exists(&self, key: uuid) -> bool{
        let conn = &mut (self.conn).unwrap();
        conn.get(format!("{}:{}",self.project_name, key))?;
        return true;
    }

    async fn delete (&self, key: uuid){
        let conn = &mut (self.conn).unwrap();
        let ret: () = conn.del(key)?;
    }
    async fn put (&self, key: uuid, ttl: prost_types::Timestamp){
        let conn = &mut (self.conn).unwrap();
        let ret : () =  conn.set_ex(format!("{}:{}",self.project_name, key),"",ttl)?;
    }

    async fn init (&self, config: DbConfig) -> dyn IDatabase{
        self.client = redis::Client::open(config.url);
        self.project_name = DbConfig.project_name;
    }
}