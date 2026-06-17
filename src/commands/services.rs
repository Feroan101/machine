use anyhow::Result;
use std::process::Command;
use colored::*;

pub async fn run(json: bool, _verbose: bool) -> Result<()> {
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


    if json {
        let result = serde_json::json!({
            "failed_services": failed.lines().filter(|l| !l.trim().is_empty()).collect::<Vec<_>>()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
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
    }

    // Check for slow services
    let slow_output = Command::new("systemd-analyze")
        .arg("blame")
        .output();
    
    if let Ok(out) = slow_output {
        let blame = String::from_utf8_lossy(&out.stdout);
        println!("\n{}", "Top slow-starting services:".yellow().bold());
        for line in blame.lines().take(5) {
            println!("  • {}", line.trim());
        }
    }

    println!("\n{} Run '{}' for more details.", "TIP:".blue(), "journalctl -xe".bold());

    Ok(())
}
