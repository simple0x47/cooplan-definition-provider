use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct LatestDefinitionUpdaterConfig {
    pub update_interval_seconds: u64,

    pub retry_count: u32,
    pub retry_interval_seconds: u64,
}
