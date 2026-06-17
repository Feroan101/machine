use anyhow::Result;
use std::process::Command;
use colored::*;

pub async fn run(json: bool, _verbose: bool) -> Result<()> {
    // 1. systemd-analyze
    let analyze = Command::new("systemd-analyze").output();

    if json {
        let mut results = serde_json::Map::new();
        if let Ok(out) = analyze {
            results.insert("summary".to_string(), serde_json::Value::String(String::from_utf8_lossy(&out.stdout).trim().to_string()));
        }
        println!("{}", serde_json::to_string_pretty(&results)?);
        return Ok(());
    }

    println!("\n{}", " BOOT PERFORMANCE ".on_cyan().black().bold());

    match analyze {
        Ok(out) if out.status.success() => {
            let result = String::from_utf8_lossy(&out.stdout);
            println!("{}", result.trim());
            
            // Critical chain
            let chain = Command::new("systemd-analyze").arg("critical-chain").output();
            if let Ok(c_out) = chain {
                println!("\n{}", "Critical Chain:".bold());
                println!("{}", String::from_utf8_lossy(&c_out.stdout).trim());
            }
        },
        _ => {
            println!("{}", "systemd-analyze not available. Falling back to /proc/uptime...".dimmed());
            let uptime = std::fs::read_to_string("/proc/uptime")?;
            let boot_time = uptime.split_whitespace().next().unwrap_or("0").parse::<f32>().unwrap_or(0.0);
            println!("System has been up for {:.2} seconds.", boot_time);
        }
    }

    // 2. Kernel version & boot params (optional additional info)
    let version = std::fs::read_to_string("/proc/version")?;
    println!("\n{} {}", "Kernel:".bold(), version.split_whitespace().nth(2).unwrap_or("Unknown"));

    Ok(())
}
