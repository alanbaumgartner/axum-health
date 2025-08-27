use {
    crate::service::{HealthDetail, HealthIndicator},
    async_trait::async_trait,
    std::{path::PathBuf, sync::Arc},
    tokio::sync::Mutex,
};

pub struct DiskSpaceHealthIndicator {
    path: PathBuf,
    threshold: u64,
    info: Arc<Mutex<sysinfo::Disks>>,
}

pub struct PingHealthIndicator;

#[async_trait]
impl HealthIndicator for PingHealthIndicator {
    fn name(&self) -> String {
        "ping".to_owned()
    }

    async fn details(&self) -> HealthDetail {
        HealthDetail::up()
    }
}

#[async_trait]
impl HealthIndicator for DiskSpaceHealthIndicator {
    fn name(&self) -> String {
        "disk_space".to_owned()
    }

    async fn details(&self) -> HealthDetail {
        let mut info = self.info.lock().await;

        info.refresh(true);

        let disk = info
            .list()
            .iter()
            .filter(|disk| self.path.starts_with(disk.mount_point()))
            .next();

        match disk {
            Some(disk) => {
                let available_space = disk.available_space();

                let detail = if available_space >= self.threshold {
                    HealthDetail::up()
                } else {
                    HealthDetail::down()
                };

                let total_space = disk.total_space();

                detail
                    .with_detail("total", total_space)
                    .with_detail("free", available_space)
                    .with_detail("threshold", self.threshold)
                    .with_detail("exists", true)
            }
            None => HealthDetail::down()
                .with_detail("threshold", self.threshold)
                .with_detail("exists", false),
        }
    }
}
