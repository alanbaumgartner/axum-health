use {
    axum_health::prelude::{HealthDetail, HealthDetails, HealthStatus},
    std::collections::BTreeMap,
    testcontainers_modules::testcontainers::{runners::AsyncRunner, ContainerAsync},
};

#[cfg(feature = "redis")]
use testcontainers_modules::redis::Redis;

#[cfg(feature = "mongo")]
use testcontainers_modules::mongo::Mongo;

#[cfg(feature = "elasticsearch")]
use testcontainers_modules::elastic_search::ElasticSearch;

#[cfg(any(feature = "diesel", feature = "sea-orm", feature = "sqlx"))]
use testcontainers_modules::{mysql::Mysql, postgres::Postgres};

#[cfg(feature = "neo4j")]
use testcontainers_modules::{neo4j::Neo4j, neo4j::Neo4jImage};

#[cfg(feature = "cassandra")]
use testcontainers_modules::scylladb::ScyllaDB;

#[cfg(any(feature = "diesel", feature = "sea-orm", feature = "sqlx"))]
pub async fn get_mysql_container() -> (String, ContainerAsync<Mysql>) {
    let container = Mysql::default().start().await.unwrap();

    let url = format!(
        "mysql://root@{}:{}/test",
        container.get_host().await.unwrap(),
        container.get_host_port_ipv4(3306).await.unwrap()
    );

    (url, container)
}

#[cfg(any(feature = "diesel", feature = "sea-orm", feature = "sqlx"))]
pub async fn get_postgres_container() -> (String, ContainerAsync<Postgres>) {
    let container = Postgres::default().start().await.unwrap();

    let url = format!(
        "postgresql://postgres:postgres@{}:{}/postgres",
        container.get_host().await.unwrap(),
        container.get_host_port_ipv4(5432).await.unwrap()
    );

    (url, container)
}

#[cfg(feature = "cassandra")]
pub async fn get_cassandra_container() -> (String, ContainerAsync<ScyllaDB>) {
    let container = ScyllaDB::default().start().await.unwrap();

    let host = container.get_host().await.unwrap().to_string();
    let port = container.get_host_port_ipv4(9042).await.unwrap();

    let url = format!("{host}:{port}");

    (url, container)
}

#[cfg(feature = "elasticsearch")]
pub async fn get_elasticsearch_container() -> (String, ContainerAsync<ElasticSearch>) {
    let container = ElasticSearch::default().start().await.unwrap();

    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(9200).await.unwrap();

    let url = format!("http://{host}:{port}");
    (url, container)
}

#[cfg(feature = "redis")]
pub async fn get_redis_container() -> (String, ContainerAsync<Redis>) {
    let container = Redis::default().start().await.unwrap();

    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(6379).await.unwrap();
    let url = format!("redis://{host}:{port}");

    (url, container)
}

#[cfg(feature = "mongo")]
pub async fn get_mongo_container() -> (String, ContainerAsync<Mongo>) {
    let container = Mongo::default().start().await.unwrap();

    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(27017).await.unwrap();
    let url = format!("mongodb://{host}:{port}/");

    (url, container)
}

#[cfg(feature = "neo4j")]
pub async fn get_neo4j_container() -> (String, ContainerAsync<Neo4jImage>) {
    let container = Neo4j::default().start().await.unwrap();

    let url = format!(
        "{}:{}",
        container.get_host().await.unwrap(),
        container.image().bolt_port_ipv4().unwrap()
    );

    (url, container)
}

pub fn health_details(name: &str, status: HealthStatus) -> HealthDetails {
    HealthDetails {
        status: status.clone(),
        components: BTreeMap::from_iter([(name.to_owned(), HealthDetail::new(status))]),
    }
}
