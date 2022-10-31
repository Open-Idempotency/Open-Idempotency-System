use cassandra_cpp::Cluster;

struct CassandraClient {
    client: cassandra_cpp::Session,
    project_name: str,
    table_name: str,
}

impl IDatabase for CassandraClient {
    async fn exists(&self, key: uuid) -> bool{

        return true
    }
    async fn delete (&self, key: uuid){

    }
    async fn put (&self, key: uuid, ttl: prost_types::Timestamp){

    }
    async fn init (&self, config: DbConfig) -> dyn IDatabase{
        let mut cluster = Cluster::default();
        cluster.set_contact_points(config.url).unwrap();
        self.client = cluster.connect_keyspace(config.keyspace).unwrap();
        self.project_name = config.project_name;
        self.table_name = config.table_name;
    }
}


