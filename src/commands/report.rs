use anyhow::Result;
use colored::*;
use crate::commands;
use crate::core::analysis::Analyzer;
use chrono::Local;

pub async fn run(output: Option<String>, _json: bool, _verbose: bool) -> Result<()> {
    println!("\n{}", " COMPREHENSIVE REPORT ".on_white().black().bold());
    println!("Generating full system diagnostic report. This may take a moment...\n");

    let mut report_content = String::new();
    report_content.push_str("MACHINE SYSTEM REPORT\n");
    report_content.push_str(&format!("Generated at: {}\n", Local::now().to_rfc2822()));
    report_content.push_str("====================================\n\n");

    if let Some(path) = output {
        println!("Saving report to: {}", path.cyan());
        
        let mut report = String::new();
        report.push_str("MACHINE SYSTEM REPORT\n");
        report.push_str("=====================\n\n");
        
        let mut analyzer = Analyzer::new();
        let snap = analyzer.get_snapshot();
        
        report.push_str(&format!("Hostname: {}\n", snap.hostname));
        report.push_str(&format!("OS: {}\n", snap.os));
        report.push_str(&format!("Kernel: {}\n", snap.kernel));
        report.push_str(&format!("Uptime: {}s\n\n", snap.uptime));
        
        report.push_str("RESOURCES\n");
        report.push_str(&format!("CPU: {:.1}%\n", snap.cpu_usage));
        report.push_str(&format!("RAM: {:.1}%\n", snap.mem_usage));
        
        std::fs::write(&path, report)?;
        println!("{} Report successfully saved to {}", "SUCCESS:".green().bold(), path);
        return Ok(());
    }

    // Interactive display
    commands::status::run(false, true).await?;
    commands::pulse::run(false, false).await?;
    commands::investigate::run(false, true).await?;
    commands::services::run(false, false).await?;
    commands::security::run(false, false).await?;
    commands::anomalies::run(false, false).await?;

    Ok(())
}
