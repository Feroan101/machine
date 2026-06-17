use anyhow::Result;
use crate::core::analysis::Analyzer;
use colored::*;
use tokio::time::{sleep, Duration};

pub async fn run(_json: bool, _verbose: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    let mut last_cpu = 0.0;

    println!("Entering watch mode. Press Ctrl+C to exit.");
    sleep(Duration::from_secs(1)).await;

    loop {
        let snapshot = analyzer.get_snapshot();
        
        // Clear screen and move cursor to top
        print!("\x1B[2J\x1B[1;1H");
        
        println!("{}", " MACHINE WATCH ".on_blue().white().bold());
        println!("{}", "Real-time system interpretation".dimmed());
        println!();

        // Interpret CPU
        let cpu_status = if snapshot.cpu_usage > 80.0 {
            "CPU spike detected, system under heavy load".red().bold()
        } else if snapshot.cpu_usage > 50.0 {
            "CPU usage elevated".yellow()
        } else if snapshot.cpu_usage < 10.0 {
            "System idle".green()
        } else if snapshot.cpu_usage < last_cpu - 5.0 {
            "CPU stabilizing after brief spike".cyan()
        } else {
            "CPU usage normal".dimmed()
        };

        // Interpret Memory
        let mem_status = if snapshot.mem_usage > 90.0 {
            "Memory pressure critical - consider closing applications".on_red().white().bold()
        } else if snapshot.mem_usage > 75.0 {
            "Memory usage increasing slightly".yellow()
        } else {
            "Memory usage stable".green()
        };

        println!("• {}", cpu_status);
        println!("• {}", mem_status);
        
        println!("\nMetrics:");
        println!("  CPU: {:.1}%", snapshot.cpu_usage);
        println!("  RAM: {:.1}%", snapshot.mem_usage);
        println!("  Load: {:.2}, {:.2}, {:.2}", snapshot.load_avg.0, snapshot.load_avg.1, snapshot.load_avg.2);

        last_cpu = snapshot.cpu_usage;
        sleep(Duration::from_secs(2)).await;
    }
}
