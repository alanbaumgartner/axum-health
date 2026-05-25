use {
    crate::prelude::{HealthDetail, HealthIndicator},
    async_trait::async_trait,
    neo4rs::Graph,
};

const CYPHER: &str = "CALL dbms.components() YIELD versions, name, edition WHERE name = 'Neo4j Kernel' RETURN edition, versions[0] as version";

#[async_trait]
impl HealthIndicator for Graph {
    fn name(&self) -> String {
        String::from("neo4j")
    }

    async fn details(&self) -> HealthDetail {
        let result = self.execute(CYPHER).await;

        match result {
            Ok(mut result) => {
                let Ok(row) = result.single().await else {
                    return HealthDetail::down();
                };

                let Ok(version) = row.get::<String>("version") else {
                    return HealthDetail::down();
                };

                let Ok(edition) = row.get::<String>("edition") else {
                    return HealthDetail::down();
                };

                #[cfg(feature = "neo4j-summary")]
                let Ok(summary) = result.finish().await else {
                    return HealthDetail::down();
                };

                let detail = HealthDetail::up()
                    .with_detail("version", version)
                    .with_detail("edition", edition);

                #[cfg(feature = "neo4j-summary")]
                match summary.db {
                    Some(db) => detail.with_detail("database", db),
                    None => detail,
                }

                #[cfg(not(feature = "neo4j-summary"))]
                detail
            }
            Err(_) => HealthDetail::down(),
        }
    }
}
