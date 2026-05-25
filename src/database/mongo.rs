use {
    crate::{indicator::HealthDetail, prelude::HealthIndicator},
    async_trait::async_trait,
    mongodb::{
        Client,
        bson::{Document, bson},
    },
    std::cmp::max,
};

#[async_trait]
impl HealthIndicator for Client {
    fn name(&self) -> String {
        "mongo".to_owned()
    }

    async fn details(&self) -> HealthDetail {
        let hello: Document = Document::from_iter([(String::from("hello"), bson!(1))]);

        match self.list_databases().await {
            Ok(dbs) => {
                let mut databases = vec![];
                let mut max_wire_version = 0;

                for db in dbs {
                    let name = db.name;

                    match self.database(&name).run_command(hello.clone()).await {
                        Ok(result) => {
                            if let Some(max_wire_version_bson) = result.get("maxWireVersion") {
                                let new_max_wire_version =
                                    max_wire_version_bson.as_i32().unwrap_or_default();
                                max_wire_version = max(max_wire_version, new_max_wire_version);
                            }
                            databases.push(name);
                        }
                        Err(_) => return HealthDetail::down(),
                    }
                }

                HealthDetail::up()
                    .with_detail("max_wire_version", max_wire_version)
                    .with_detail("databases", databases)
            }
            Err(_) => HealthDetail::down(),
        }
    }
}
