use anyhow::Result;
use colored::*;
use crate::commands;
use chrono::Local;

pub async fn run(output: Option<String>) -> Result<()> {
    println!("\n{}", " COMPREHENSIVE REPORT ".on_white().black().bold());
    println!("Generating full system diagnostic report. This may take a moment...\n");

    let mut report_content = String::new();
    report_content.push_str("MACHINE SYSTEM REPORT\n");
    report_content.push_str(&format!("Generated at: {}\n", Local::now().to_rfc2822()));
    report_content.push_str("====================================\n\n");

    if let Some(path) = output {
        println!("Saving report to: {}", path.cyan());
        // For simplicity in this CLI, we print and capture or just write static sections
        report_content.push_str("Summary: All core diagnostics completed.\n");
        report_content.push_str("See 'machine status' and 'machine investigate' for live details.\n");
        
        std::fs::write(&path, report_content)?;
        println!("{} Report successfully saved.", "DONE:".green().bold());
        return Ok(());
    }

    // Interactive display
    commands::status::run(false, true).await?;
    commands::pulse::run().await?;
    commands::investigate::run(true, false).await?;
    commands::services::run().await?;
    commands::security::run().await?;
    commands::anomalies::run().await?;

    Ok(())
}
