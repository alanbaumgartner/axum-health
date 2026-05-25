use {
    crate::{indicator::HealthDetail, prelude::HealthIndicator},
    async_trait::async_trait,
    mongodb::{
        bson::{bson, Document},
        Client,
    },
};

#[async_trait]
impl HealthIndicator for Client {
    fn name(&self) -> String {
        "mongo".to_owned()
    }

    async fn details(&self) -> HealthDetail {
        let hello: Document = Document::from_iter([(String::from("hello"), bson!(1))]);

        let mut databases = vec![];
        let mut max_wire_version = 0;

        match self.list_databases().await {
            Ok(dbs) => {
                for db in dbs {
                    let name = db.name;

                    match self.database(&name).run_command(hello.clone()).await {
                        Ok(result) => {
                            let max_wire_version_bson = result.get("maxWireVersion").unwrap();
                            max_wire_version = max_wire_version_bson.as_i32().unwrap();

                            databases.push(name);
                        }
                        Err(_) => return HealthDetail::down(),
                    }
                }
            }
            Err(_) => return HealthDetail::down(),
        }

        HealthDetail::up()
            .with_detail("max_wire_version", max_wire_version)
            .with_detail("databases", serde_json::to_string(&databases).unwrap())
    }
}
