use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct CpuInfo {
    pub cpu: f32,
    pub cores: HashMap<String, f32>,
}

#[derive(Serialize)]
pub struct RamInfo {
    pub used_ram: u64,
    pub total_ram: u64,
}

#[derive(Serialize)]
pub struct MonitorInfo {
    pub cpu: CpuInfo,
    pub ram: RamInfo,
}
