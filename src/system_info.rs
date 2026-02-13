use std::collections::HashMap;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use crate::models::{CpuInfo, RamInfo};

pub fn get_cpu_info(sys: &mut System) -> CpuInfo {
    let global_cpu = sys.global_cpu_info().cpu_usage();
    let mut cores = HashMap::new();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        cores.insert(i.to_string(), cpu.cpu_usage());
    }

    CpuInfo {
        cpu: global_cpu,
        cores,
    }
}

pub fn get_ram_info(sys: &mut System) -> RamInfo {
    RamInfo {
        used_ram: sys.used_memory(),
        total_ram: sys.total_memory(),
    }
}

pub fn refresh_cpu(sys: &mut System) {
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
}

pub fn refresh_ram(sys: &mut System) {
    sys.refresh_memory_specifics(MemoryRefreshKind::everything());
}

pub fn refresh_all(sys: &mut System) {
    sys.refresh_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()).with_memory(MemoryRefreshKind::everything()));
}
