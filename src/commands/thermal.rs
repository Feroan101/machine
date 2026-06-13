use anyhow::Result;
use colored::*;
use std::fs;

pub async fn run() -> Result<()> {
    println!("\n{}", " THERMAL ANALYSIS ".on_red().white().bold());

    let thermal_path = "/sys/class/thermal";
    if !std::path::Path::new(thermal_path).exists() {
        println!("No thermal information found.");
        return Ok(());
    }

    let zones = fs::read_dir(thermal_path)?;
    for zone in zones {
        let path = zone?.path();
        let name = path.file_name().unwrap().to_string_lossy();
        if name.starts_with("thermal_zone") {
            let type_name = fs::read_to_string(path.join("type")).unwrap_or_else(|_| "Unknown".to_string());
            let temp_str = fs::read_to_string(path.join("temp")).unwrap_or_else(|_| "0".to_string());
            let temp = temp_str.trim().parse::<f32>().unwrap_or(0.0) / 1000.0;

            let color = if temp > 80.0 { "red" } else if temp > 60.0 { "yellow" } else { "green" };
            let colored_temp = format!("{:.1}°C", temp);
            
            println!("{:<20} {}", type_name.trim().bold(), if color == "red" { colored_temp.red().bold() } else if color == "yellow" { colored_temp.yellow() } else { colored_temp.green() });
        }
    }

    Ok(())
}
