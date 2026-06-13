use anyhow::Result;
use colored::*;
use std::process::Command;

pub async fn run() -> Result<()> {
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
            let process = parts.get(6).unwrap_or(&"Unknown");
            println!("{:<8} {:<20} {:<10}", proto, addr, process);
        }
    }

    Ok(())
}
