use anyhow::Result;
use colored::*;
use std::fs;

pub async fn run() -> Result<()> {
    println!("\n{}", " BATTERY HEALTH ".on_green().black().bold());

    let power_path = "/sys/class/power_supply";
    if !std::path::Path::new(power_path).exists() {
        println!("No power supply information found. Desktop system?");
        return Ok(());
    }

    let supplies = fs::read_dir(power_path)?;
    for supply in supplies {
        let path = supply?.path();
        let name = path.file_name().unwrap().to_string_lossy();
        if name.starts_with("BAT") {
            let capacity = fs::read_to_string(path.join("capacity")).unwrap_or_default();
            let status = fs::read_to_string(path.join("status")).unwrap_or_default();
            let model = fs::read_to_string(path.join("model_name")).unwrap_or_else(|_| "Unknown".to_string());

            println!("{:<15} {}", "Battery:".bold(), model.trim());
            println!("{:<15} {}%", "Level:".bold(), capacity.trim());
            println!("{:<15} {}", "Status:".bold(), status.trim());
        }
    }

    Ok(())
}
