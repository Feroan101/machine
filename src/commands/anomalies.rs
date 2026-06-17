use anyhow::Result;
use crate::storage::db::StorageManager;
use crate::core::analysis::{Analyzer, SystemSnapshot};
use colored::*;

pub async fn run(json: bool, _verbose: bool) -> Result<()> {
    println!("\n{}", " ANOMALY DETECTION ".on_yellow().black().bold());
    println!("Scanning for unusual system activity compared to history...\n");

    let mut analyzer = Analyzer::new();
    let current = analyzer.get_snapshot();
    
    let storage = StorageManager::new().await?;
    let snapshots_data = storage.list_snapshots(30).await?;
    
    if snapshots_data.is_empty() {
        if json {
            println!("{}", serde_json::json!({"status": "no_history", "current": current}));
            return Ok(());
        }
        println!("{}", "No historical data found. Analyzing current state only...".bright_black());
        check_anomalies_local(&current);
        return Ok(());
    }

    let mut history = Vec::new();
    for (id, _) in snapshots_data {
        if let Some(data) = storage.get_snapshot(&id).await? {
            if let Ok(snap) = serde_json::from_str::<SystemSnapshot>(&data) {
                history.push(snap);
            }
        }
    }

    let avg_cpu: f32 = history.iter().map(|s| s.cpu_usage).sum::<f32>() / history.len() as f32;
    let avg_mem: f32 = history.iter().map(|s| s.mem_usage).sum::<f32>() / history.len() as f32;

    let mut anomalies = Vec::new();
    if current.cpu_usage > avg_cpu + 20.0 {
        anomalies.push(format!("Significant CPU spike detected ({:.1}% vs avg {:.1}%)", current.cpu_usage, avg_cpu));
    }
    if current.mem_usage > avg_mem + 15.0 {
        anomalies.push(format!("Memory usage is unusually high ({:.1}% vs avg {:.1}%)", current.mem_usage, avg_mem));
    }

    if json {
        let result = serde_json::json!({
            "anomalies": anomalies,
            "averages": {"cpu": avg_cpu, "memory": avg_mem},
            "current": {"cpu": current.cpu_usage, "memory": current.mem_usage}
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    if anomalies.is_empty() {
        println!("{} No major anomalies detected compared to historical data.", "OK:".green().bold());
    } else {
        for a in anomalies {
            println!("{} {}", "ANOMALY:".red().bold(), a);
        }
    }

    Ok(())
}

fn check_anomalies_local(snap: &SystemSnapshot) {
    if snap.cpu_usage > 70.0 {
        println!("{} High CPU usage detected.", "INFO:".yellow());
    }
    if snap.mem_usage > 85.0 {
        println!("{} High memory usage detected.", "INFO:".yellow());
    }
}
