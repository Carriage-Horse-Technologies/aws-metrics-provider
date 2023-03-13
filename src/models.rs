use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Ec2MetricStatistics {
    pub cpuutilization: f64,
    pub disk_read_bytes: f64,
    pub disk_write_bytes: f64,
    pub network_in: f64,
    pub network_out: f64,
}
