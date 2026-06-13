use anyhow::Result;
use std::process::Command;
use colored::*;

pub async fn run() -> Result<()> {
    println!("\n{}", " SYSTEM SERVICES ".on_blue().black().bold());

    // Overall stats
    let total_output = Command::new("systemctl")
        .arg("count-units")
        .output().ok();
    
    let failed_output = Command::new("systemctl")
        .args(["--failed", "--no-legend"])
        .output()?;
    
    let failed = String::from_utf8_lossy(&failed_output.stdout);
    
    if let Some(out) = total_output {
        let stats = String::from_utf8_lossy(&out.stdout);
        println!("{}", stats.trim().bright_black());
    }

    if failed.trim().is_empty() {
        println!("\n{}", "All system services are operational.".green().bold());
    } else {
        println!("\n{}", "Failed services detected:".red().bold());
        for line in failed.lines() {
            if !line.trim().is_empty() {
                println!("  • {}", line.trim());
            }
        }
        println!("\n{} Run '{}' for more details.", "TIP:".blue(), "journalctl -xe".bold());
    }

    Ok(())
}
