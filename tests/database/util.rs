use testcontainers_modules::{
    mysql::Mysql,
    postgres::Postgres,
    testcontainers::{runners::AsyncRunner, ContainerAsync},
};

pub async fn get_mysql_container() -> (String, ContainerAsync<Mysql>) {
    let container = Mysql::default().start().await.unwrap();
    container.start().await.unwrap();

    let url = format!(
        "mysql://root@{}:{}/test",
        container.get_host().await.unwrap(),
        container.get_host_port_ipv4(3306).await.unwrap()
    );

    (url, container)
}

pub async fn get_postgres_container() -> (String, ContainerAsync<Postgres>) {
    let container = Postgres::default().start().await.unwrap();
    container.start().await.unwrap();

    let url = format!(
        "postgresql://postgres:postgres@{}:{}/postgres",
        container.get_host().await.unwrap(),
        container.get_host_port_ipv4(5432).await.unwrap()
    );

    (url, container)
}
