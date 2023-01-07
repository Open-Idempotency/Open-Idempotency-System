use cassandra_cpp::Cluster;
use std::time::Duration;

pub struct CassandraClient {
    client: cassandra_cpp::Session,
    table_name: str,
}

impl IDatabase for CassandraClient {
    async fn exists(&self, key: uuid, app_id: String) -> bool{

        return true
    }
    async fn delete (&self, key: uuid, app_id: String){

    }
    async fn put (&self, key: uuid, app_id: String, ttl: Duration){

    }
    async fn init (&mut self, config: DbConfig) -> dyn IDatabase{
        let mut cluster = Cluster::default();
        cluster.set_contact_points(config.url).unwrap();
        self.client = cluster.connect_keyspace(config.keyspace).unwrap();
        self.project_name = config.project_name;
        self.table_name = config.table_name;
    }
}


