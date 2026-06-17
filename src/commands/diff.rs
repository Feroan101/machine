use anyhow::{Result, Context};
use crate::core::analysis::SystemSnapshot;
use crate::storage::db::StorageManager;
use colored::*;

pub async fn run(id1: String, id2: String, _json: bool, _verbose: bool) -> Result<()> {
    let storage = StorageManager::new().await?;
    
    let data1 = storage.get_snapshot(&id1).await?.context(format!("Snapshot {} not found", id1))?;
    let data2 = storage.get_snapshot(&id2).await?.context(format!("Snapshot {} not found", id2))?;

    let snap1: SystemSnapshot = serde_json::from_str(&data1)?;
    let snap2: SystemSnapshot = serde_json::from_str(&data2)?;

    println!("\n{} Comparing {} vs {}", " SYSTEM DIFF ".on_yellow().black().bold(), id1.cyan(), id2.cyan());

    // Compare Resources
    println!("\n{}", "--- Resource Changes ---".bright_black());
    print_diff("CPU Usage", snap1.cpu_usage, snap2.cpu_usage, "%");
    print_diff("RAM Usage", snap1.mem_usage, snap2.mem_usage, "%");
    print_diff("Swap Usage", snap1.swap_usage, snap2.swap_usage, "%");

    println!("\n{}", "--- System Info ---".bright_black());
    if snap1.kernel != snap2.kernel {
        println!("{:<15} {} -> {}", "Kernel:".bold(), snap1.kernel.red(), snap2.kernel.green());
    } else {
        println!("{:<15} {}", "Kernel:".bold(), snap1.kernel.bright_black());
    }

    Ok(())
}

fn print_diff(label: &str, v1: f32, v2: f32, unit: &str) {
    let diff = v2 - v1;
    let color = if diff > 5.0 { "red" } else if diff < -5.0 { "green" } else { "white" };
    let arrow = if diff > 0.1 { "↑" } else if diff < -0.1 { "↓" } else { "=" };
    
    let line = format!("{:<15} {:.1}{} -> {:.1}{} ({}{:.1}{})", 
        label.bold(), 
        v1, unit, 
        v2, unit,
        arrow, diff.abs(), unit);
    
    if color == "red" {
        println!("{}", line.red());
    } else if color == "green" {
        println!("{}", line.green());
    } else {
        println!("{}", line);
    }
}
