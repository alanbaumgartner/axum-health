use {
    crate::indicator::{HealthDetail, HealthIndicator},
    async_trait::async_trait,
};

#[cfg(feature = "disk")]
pub use disk::*;

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

#[cfg(feature = "disk")]
mod disk {
    use {
        crate::prelude::{HealthDetail, HealthIndicator},
        async_trait::async_trait,
        std::{path::PathBuf, sync::Arc},
        tokio::sync::Mutex,
    };

    pub struct DiskSpaceHealthIndicator {
        path: PathBuf,
        threshold: u64,
        info: Arc<Mutex<sysinfo::Disks>>,
    }

    impl DiskSpaceHealthIndicator {
        pub fn new(path: PathBuf, threshold: u64) -> Self {
            Self {
                path,
                threshold,
                info: Arc::new(Mutex::new(sysinfo::Disks::new())),
            }
        }
    }

    #[async_trait]
    impl HealthIndicator for DiskSpaceHealthIndicator {
        fn name(&self) -> String {
            String::from("disk_space")
        }

        async fn details(&self) -> HealthDetail {
            let mut info = self.info.lock().await;

            info.refresh(true);

            let disk = info
                .list()
                .iter()
                .find(|disk| self.path.starts_with(disk.mount_point()));

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
}
