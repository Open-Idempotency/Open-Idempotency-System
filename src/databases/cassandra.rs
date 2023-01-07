use std::sync::Arc;
use cassandra_cpp::Cluster;
use std::time::Duration;
use crate::databases::database::{DbConfig, IDatabase};

pub struct CassandraClient {
    client: cassandra_cpp::Session,
    table_name: String,
}

impl IDatabase for CassandraClient {
    fn exists(&self, key: String, app_id: String) -> bool{

        return true
    }
    fn delete (&self, key: String, app_id: String){

    }
    fn put (&self, key: String, app_id: String, ttl: Duration){

    }

}

impl CassandraClient {
    fn new (config: DbConfig) -> Arc<dyn IDatabase> {
        let mut cluster = Cluster::default();
        cluster.set_contact_points(config.url).unwrap();
        let c = CassandraClient {
            client: cluster.connect_keyspace(config.keyspace).unwrap(),
            table_name: config.table_name.unwrap()
        };
        return Arc::new(c);
    }
}

