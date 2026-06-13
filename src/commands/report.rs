use anyhow::Result;
use colored::*;
use crate::commands;

pub async fn run(output: Option<String>) -> Result<()> {
    println!("\n{}", " COMPREHENSIVE REPORT ".on_white().black().bold());
    println!("Generating full system diagnostic report. This may take a moment...\n");

    if let Some(path) = output {
        println!("Saving report to: {}", path.cyan());
        // Simple stub for now - real implementation would capture all results
        std::fs::write(&path, "Machine System Report\nGenerated at: ...")?;
        return Ok(());
    }

    commands::status::run(false, true).await?;
    commands::investigate::run(true, false).await?;
    commands::services::run().await?;
    commands::security::run().await?;

    Ok(())
}
