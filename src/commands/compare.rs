use anyhow::{Result, Context};
use colored::*;
use crate::storage::db::StorageManager;
use crate::core::analysis::{Analyzer, SystemSnapshot};

pub async fn run(period: Option<String>, json: bool, _verbose: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    let current = analyzer.get_snapshot();
    let storage = StorageManager::new().await?;

    println!("\n{}", " SYSTEM COMPARE ".on_blue().white().bold());

    let snapshot_id = if let Some(p) = period {
        // Try to find a snapshot matching common strings or just use the latest
        println!("Looking for snapshots from '{}'...", p);
        storage.list_snapshots(1).await?.first().map(|(id, _)| id.clone())
    } else {
        storage.list_snapshots(1).await?.first().map(|(id, _)| id.clone())
    };

    if let Some(id) = snapshot_id {
        let data = storage.get_snapshot(&id).await?.context("Snapshot data missing")?;
        let snap: SystemSnapshot = serde_json::from_str(&data)?;

        if json {
            let result = serde_json::json!({
                "historical_id": id,
                "current": current,
                "historical": snap
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
            return Ok(());
        }

        println!("Comparing current state against historical snapshot: {}\n", id.cyan());
        
        // We reuse the diff logic but compare Current vs Historical
        println!("{}", "--- Changes ---".bright_black());
        print_res_diff("CPU Usage", snap.cpu_usage, current.cpu_usage, "%");
        print_res_diff("RAM Usage", snap.mem_usage, current.mem_usage, "%");
        
        if snap.kernel != current.kernel {
            println!("{:<15} {} -> {}", "Kernel:".bold(), snap.kernel.red(), current.kernel.green());
        }
    } else {
        println!("{}", "No historical snapshots found to compare against.".yellow());
        println!("Run 'machine snapshot' to record current state for future comparison.");
    }

    Ok(())
}

fn print_res_diff(label: &str, v1: f32, v2: f32, unit: &str) {
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
