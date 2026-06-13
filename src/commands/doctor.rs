use anyhow::Result;
use crate::core::analysis::Analyzer;
use colored::*;
use sysinfo::Disks;

pub async fn run() -> Result<()> {
    let mut analyzer = Analyzer::new();
    let snapshot = analyzer.get_snapshot();

    println!("\n{}", " MACHINE DOCTOR ".on_green().black().bold());
    println!("Checking system health score...\n");

    let cpu_score = (100.0f32 - snapshot.cpu_usage).clamp(0.0f32, 100.0f32);
    let mem_score = (100.0f32 - snapshot.mem_usage).clamp(0.0f32, 100.0f32);
    
    // Storage Health
    let disks = Disks::new_with_refreshed_list();
    let mut disk_score = 100.0f32;
    for disk in &disks {
        let usage = 100.0 - (disk.available_space() as f32 / disk.total_space() as f32 * 100.0);
        if usage > 90.0 {
            disk_score = disk_score.min(20.0);
        } else if usage > 75.0 {
            disk_score = disk_score.min(60.0);
        }
    }

    let health_score = (cpu_score + mem_score + disk_score) / 3.0;

    println!("{:<15} {:.1}/100", "CPU Health:".bold(), cpu_score);
    println!("{:<15} {:.1}/100", "Memory Health:".bold(), mem_score);
    println!("{:<15} {:.1}/100", "Storage Health:".bold(), disk_score);

    println!("\n{} {:.0}/100", "FINAL HEALTH SCORE:".bold(), health_score);

    if health_score > 90.0 {
        println!("{}", "Your system is in excellent shape. No immediate action required.".green());
    } else if health_score > 70.0 {
        println!("{}", "Your system is healthy but showing some load. Monitor active processes.".yellow());
    } else {
        println!("{}", "Your system requires attention soon. Check 'machine investigate' for details.".red());
    }

    // Repair advice
    if health_score < 80.0 {
        println!("\n{}", "PROPOSED ACTIONS:".bold());
        if cpu_score < 70.0 {
            println!("  • Identify and terminate high CPU consumers using '{}'", "machine top".bold());
        }
        if mem_score < 70.0 {
            println!("  • Clear RAM by closing unused browser tabs or using '{}'", "machine clean --mem".bold());
        }
        if disk_score < 70.0 {
            println!("  • Free up disk space by running '{}'", "machine clean".bold());
        }
    }

    Ok(())
}
