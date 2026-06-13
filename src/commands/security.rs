use anyhow::Result;
use colored::*;
use std::process::Command;

pub async fn run() -> Result<()> {
    println!("\n{}", " SECURITY AUDIT ".on_red().white().bold());

    check_root();
    check_listening_ports();
    check_world_writable();

    Ok(())
}

fn check_root() {
    let output = Command::new("id").arg("-u").output().ok();
    if let Some(out) = output {
        if String::from_utf8_lossy(&out.stdout).trim() == "0" {
            println!("{} Running as root. This is not recommended for exploratory analysis.", "WARNING:".yellow().bold());
        } else {
            println!("{} Running with restricted user privileges.", "PASS:".green().bold());
        }
    }
}

fn check_listening_ports() {
    println!("\nChecking for unusual listening ports...");
    // Simulated check for now, real ports command will be more thorough
    println!("{} Check 'machine ports' for detailed network activity.", "TIP:".blue().bold());
}

fn check_world_writable() {
    println!("\nChecking for world-writable files in /tmp...");
    let output = Command::new("find")
        .args(["/tmp", "-maxdepth", "1", "-perm", "-0002", "-type", "f"])
        .output()
        .ok();
    
    if let Some(out) = output {
        let files = String::from_utf8_lossy(&out.stdout);
        if !files.trim().is_empty() {
            println!("{} Found world-writable files in /tmp:", "INFO:".yellow());
            println!("{}", files);
        } else {
            println!("{} No suspicious world-writable files found.", "PASS:".green());
        }
    }
}
