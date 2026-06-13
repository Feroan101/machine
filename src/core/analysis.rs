use sysinfo::System;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemSnapshot {
    pub cpu_usage: f32,
    pub mem_usage: f32,
    pub swap_usage: f32,
    pub uptime: u64,
    pub load_avg: (f64, f64, f64),
    pub hostname: String,
    pub kernel: String,
    pub os: String,
}

pub struct Analyzer {
    pub sys: System,
}

impl Analyzer {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    pub fn get_snapshot(&mut self) -> SystemSnapshot {
        self.sys.refresh_all();
        
        let total_mem = self.sys.total_memory() as f32;
        let used_mem = self.sys.used_memory() as f32;
        let mem_usage = if total_mem > 0.0 { (used_mem / total_mem) * 100.0 } else { 0.0 };

        let total_swap = self.sys.total_swap() as f32;
        let used_swap = self.sys.used_swap() as f32;
        let swap_usage = if total_swap > 0.0 { (used_swap / total_swap) * 100.0 } else { 0.0 };

        let cpu_usage = self.sys.global_cpu_info().cpu_usage();
        let uptime = System::uptime();
        let load = System::load_average();

        SystemSnapshot {
            cpu_usage,
            mem_usage,
            swap_usage,
            uptime,
            load_avg: (load.one, load.five, load.fifteen),
            hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
            kernel: System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
            os: System::long_os_version().unwrap_or_else(|| "unknown".to_string()),
        }
    }

    pub fn explain_health(&self, snapshot: &SystemSnapshot) -> String {
        if snapshot.mem_usage > 90.0 {
            "Memory pressure detected. Your system is struggling to keep all active applications in RAM.".to_string()
        } else if snapshot.cpu_usage > 90.0 {
            "High CPU activity detected. One or more processes are consuming most of your processing power.".to_string()
        } else if snapshot.load_avg.0 > (self.sys.cpus().len() as f64) * 1.5 {
            "System load is very high. Tasks are queuing up faster than the CPU can handle them.".to_string()
        } else {
            "System health looks good. All resources are within normal operating parameters.".to_string()
        }
    }
}
