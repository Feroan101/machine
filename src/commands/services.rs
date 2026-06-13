use anyhow::Result;
use std::process::Command;
use colored::*;

pub async fn run() -> Result<()> {
    println!("\n{}", " SYSTEM SERVICES ".on_blue().black().bold());

    let output = Command::new("systemctl")
        .arg("--failed")
        .arg("--no-legend")
        .output()?;
    
    let failed = String::from_utf8_lossy(&output.stdout);
    
    if failed.trim().is_empty() {
        println!("{}", "All services are running correctly.".green());
    } else {
        println!("{}", "Failed services detected:".red().bold());
        println!("{}", failed);
    }

    Ok(())
}
