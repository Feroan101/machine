use anyhow::Result;
use colored::*;
use std::process::Command;

pub async fn run(json: bool, _verbose: bool) -> Result<()> {
    let output = Command::new("ss")
        .args(["-atunp"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    if json {
        let lines: Vec<_> = stdout.lines().skip(1).collect();
        println!("{}", serde_json::to_string_pretty(&lines)?);
        return Ok(());
    }

    println!("\n{}", " LISTENING PORTS ".on_cyan().black().bold());
    println!("{:<8} {:<20} {:<10}", "Proto".bold(), "Address".bold(), "Process".bold());
    println!("{}", "-".repeat(40).bright_black());

    let output = Command::new("ss")
        .args(["-tulpn"])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let proto = parts[0];
            let addr = parts[4];
            
            // Extract process info which is usually in the last part
            let process = parts.last().unwrap_or(&"Unknown");
            let clean_proc = if process.contains("users:((") {
                process.split('"').nth(1).unwrap_or(process)
            } else {
                process
            };

            println!("{:<8} {:<20} {:<10}", proto, addr, clean_proc);
        }
    }

    Ok(())
}
