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

    println!("\n{}", " NETWORK CONNECTIONS ".on_blue().white().bold());
    println!("{:<10} {:<25} {:<25} {:<15}", "State".bold(), "Local Address".bold(), "Peer Address".bold(), "Process".bold());
    println!("{}", "-".repeat(75).bright_black());

    let output = Command::new("ss")
        .args(["-atunp"])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let state = parts[0];
            let local = parts[4];
            let peer = parts[5];
            let process = parts.get(6).unwrap_or(&"-");

            println!("{:<10} {:<25} {:<25} {:<15}", 
                state.cyan(), 
                local, 
                peer, 
                process.bright_black());
        }
    }

    Ok(())
}
