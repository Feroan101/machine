use sysinfo::System;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub mem_usage: u64,
    pub uptime: u64,
}

pub fn get_process_by_target(sys: &System, target: &str) -> Option<ProcessInfo> {
    // Try as PID first
    if let Ok(pid) = target.parse::<u32>() {
        if let Some(p) = sys.process(sysinfo::Pid::from(pid as usize)) {
            return Some(ProcessInfo {
                pid: p.pid().as_u32(),
                name: p.name().to_string(),
                cpu_usage: p.cpu_usage(),
                mem_usage: p.memory(),
                uptime: p.run_time(),
            });
        }
    }

    // Try as Name
    sys.processes().values()
        .find(|p| p.name().to_lowercase().contains(&target.to_lowercase()))
        .map(|p| ProcessInfo {
            pid: p.pid().as_u32(),
            name: p.name().to_string(),
            cpu_usage: p.cpu_usage(),
            mem_usage: p.memory(),
            uptime: p.run_time(),
        })
}

pub fn get_parent_process(sys: &System, pid: u32) -> Option<(u32, String)> {
    sys.process(sysinfo::Pid::from(pid as usize))
        .and_then(|p| p.parent())
        .and_then(|parent_pid| {
            sys.process(parent_pid).map(|parent| (parent.pid().as_u32(), parent.name().to_string()))
        })
}

pub fn get_ancestry(sys: &System, pid: u32) -> Vec<(u32, String)> {
    let mut ancestry = Vec::new();
    let mut current_pid = Some(sysinfo::Pid::from(pid as usize));

    while let Some(pid) = current_pid {
        if let Some(p) = sys.process(pid) {
            ancestry.push((p.pid().as_u32(), p.name().to_string()));
            current_pid = p.parent();
        } else {
            break;
        }
    }
    
    ancestry.reverse();
    ancestry
}

pub fn get_top_processes(sys: &System, limit: usize) -> Vec<ProcessInfo> {
    let mut processes: Vec<ProcessInfo> = sys.processes().values()
        .map(|p| ProcessInfo {
            pid: p.pid().as_u32(),
            name: p.name().to_string(),
            cpu_usage: p.cpu_usage(),
            mem_usage: p.memory(),
            uptime: p.run_time(),
        })
        .collect();

    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
    processes.into_iter().take(limit).collect()
}

pub fn get_child_processes(sys: &System, ppid: u32) -> Vec<ProcessInfo> {
    sys.processes().values()
        .filter(|p| p.parent().map(|p| p.as_u32()) == Some(ppid))
        .map(|p| ProcessInfo {
            pid: p.pid().as_u32(),
            name: p.name().to_string(),
            cpu_usage: p.cpu_usage(),
            mem_usage: p.memory(),
            uptime: p.run_time(),
        })
        .collect()
}
