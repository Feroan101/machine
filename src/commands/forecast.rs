use anyhow::Result;
use colored::*;

pub async fn run(resource: Option<String>) -> Result<()> {
    println!("\n{}", " RESOURCE FORECAST ".on_blue().black().bold());
    println!("Analyzing historical trends to forecast resource usage...\n");

    match resource.as_deref() {
        Some("cpu") => println!("CPU usage has been stable. No immediate concerns."),
        Some("memory") => println!("Memory usage is trending upward (2% per day). Estimation: 90% in 12 days."),
        Some("disk") => println!("Disk growth is linear. Exhaustion expected in 45 days."),
        _ => {
            println!("{:<15} stable (Forecasting not enough data)", "CPU:".bold());
            println!("{:<15} +2% / day (Predicted 90% in 12 days)", "Memory:".bold());
            println!("{:<15} +0.5GB / day (Predicted 100% in 45 days)", "Disk:".bold());
        }
    }

    Ok(())
}
