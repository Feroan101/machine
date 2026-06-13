use anyhow::Result;
use colored::*;
use std::process::Command;
use std::time::Instant;

pub async fn run() -> Result<()> {
    println!("\n{}", " NETWORK LATENCY ".on_cyan().black().bold());

    check_dns()?;
    check_gateway()?;

    Ok(())
}

fn check_dns() -> Result<()> {
    print!("{:<15} ", "DNS Latency:".bold());
    let start = Instant::now();
    let output = Command::new("host").arg("google.com").output();
    
    if output.is_ok() && output.unwrap().status.success() {
        let duration = start.elapsed();
        println!("{:.2}ms {}", duration.as_millis(), if duration.as_millis() < 50 { "(Excellent)".green() } else { "(Normal)".yellow() });
    } else {
        println!("{}", "Failed".red());
    }
    Ok(())
}

fn check_gateway() -> Result<()> {
    print!("{:<15} ", "Gateway Ping:".bold());
    let output = Command::new("ping")
        .args(["-c", "1", "-W", "1", "8.8.8.8"]) // Simple check
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            println!("{}", "Responsive".green());
        } else {
            println!("{}", "Slow/Unresponsive".red());
        }
    } else {
        println!("{}", "Error running ping".red());
    }
    Ok(())
}
