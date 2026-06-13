use anyhow::Result;
use crate::core::analysis::Analyzer;
use colored::*;

pub async fn run() -> Result<()> {
    let mut analyzer = Analyzer::new();
    let snapshot = analyzer.get_snapshot();

    println!("\n{}", " MACHINE DOCTOR ".on_green().black().bold());
    println!("Checking system health score...\n");

    let cpu_score = (100.0f32 - snapshot.cpu_usage).clamp(0.0f32, 100.0f32);
    let mem_score = (100.0f32 - snapshot.mem_usage).clamp(0.0f32, 100.0f32);
    let health_score = (cpu_score + mem_score) / 2.0;

    println!("{:<15} {:.1}/100", "CPU Health:".bold(), cpu_score);
    println!("{:<15} {:.1}/100", "Memory Health:".bold(), mem_score);
    println!("{:<15} {:.1}/100", "Storage:".bold(), 100.0); // Stub

    println!("\n{} {:.0}/100", "FINAL HEALTH SCORE:".bold(), health_score);

    if health_score > 90.0 {
        println!("{}", "Your system is in excellent shape.".green());
    } else if health_score > 70.0 {
        println!("{}", "Your system is healthy but showing some load.".yellow());
    } else {
        println!("{}", "Your system requires attention soon.".red());
    }

    Ok(())
}
