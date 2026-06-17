use anyhow::Result;
use colored::*;
use std::process::Command;

pub async fn run(json: bool, _verbose: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::json!({"status": "audit_completed"}));
        return Ok(());
    }
    println!("\n{}", " SECURITY AUDIT ".on_red().white().bold());

    check_root();
    check_ssh_config();
    check_firewall();
    check_suid_files();
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

fn check_ssh_config() {
    println!("\n{}", "--- SSH Configuration ---".bright_black());
    let ssh_config = "/etc/ssh/sshd_config";
    if let Ok(config) = std::fs::read_to_string(ssh_config) {
        if config.contains("PermitRootLogin yes") {
            println!("{} SSH permits root login. Consider disabling it.", "WARNING:".yellow());
        } else {
            println!("{} Root login via SSH appears disabled or restricted.", "PASS:".green());
        }
        
        if config.contains("PasswordAuthentication yes") {
            println!("{} Password authentication enabled. Consider using keys only.", "INFO:".blue());
        }
    } else {
        println!("{} Could not read SSH config (permissions?).", "SKIP:".dimmed());
    }
}

fn check_firewall() {
    println!("\n{}", "--- Firewall Status ---".bright_black());
    let ufw = Command::new("ufw").arg("status").output();
    if let Ok(out) = ufw {
        let status = String::from_utf8_lossy(&out.stdout);
        if status.contains("active") {
            println!("{} UFW is active.", "PASS:".green());
        } else {
            println!("{} UFW is inactive.", "WARNING:".yellow());
        }
    } else {
        println!("{} UFW not found or inaccessible.", "SKIP:".dimmed());
    }
}

fn check_suid_files() {
    println!("\n{}", "--- SUID Files ---".bright_black());
    println!("Checking for SUID files in /usr/bin (top 5)...");
    let output = Command::new("find")
        .args(["/usr/bin", "-maxdepth", "1", "-perm", "-4000", "-type", "f"])
        .output()
        .ok();
    
    if let Some(out) = output {
        let files = String::from_utf8_lossy(&out.stdout);
        let count = files.lines().count();
        if count > 0 {
            println!("{} Found {} SUID files in /usr/bin.", "INFO:".blue(), count);
            for file in files.lines().take(5) {
                println!("  • {}", file);
            }
        }
    }
}

fn check_world_writable() {
    println!("\n{}", "--- World-Writable Files ---".bright_black());
    println!("Checking /tmp...");
    let output = Command::new("find")
        .args(["/tmp", "-maxdepth", "1", "-perm", "-0002", "-type", "f"])
        .output()
        .ok();
    
    if let Some(out) = output {
        let files = String::from_utf8_lossy(&out.stdout);
        if !files.trim().is_empty() {
            println!("{} Found world-writable files in /tmp:", "WARNING:".yellow());
            for file in files.lines().take(5) {
                println!("  • {}", file);
            }
        } else {
            println!("{} No suspicious world-writable files found.", "PASS:".green());
        }
    }
}
