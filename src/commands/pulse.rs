use anyhow::Result;
use crate::core::analysis::Analyzer;
use colored::*;

pub async fn run() -> Result<()> {
    let mut analyzer = Analyzer::new();
    let snapshot = analyzer.get_snapshot();
    
    // Single-screen compact view
    println!("\n{} {}", "MACHINE PULSE:".bold(), "System Heartbeat".dimmed());
    
    let cpu_bar = format!("{:.1}%", snapshot.cpu_usage);
    let mem_bar = format!("{:.1}%", snapshot.mem_usage);
    
    println!("  {:<8} {}", "CPU:".bold(), if snapshot.cpu_usage > 80.0 { cpu_bar.red() } else { cpu_bar.green() });
    println!("  {:<8} {}", "RAM:".bold(), if snapshot.mem_usage > 90.0 { mem_bar.red() } else { mem_bar.green() });
    
    // Simple interpretation line
    let interpretation = analyzer.explain_health(&snapshot);
    let status_line = if snapshot.cpu_usage > 90.0 || snapshot.mem_usage > 95.0 {
        "High pressure detected".on_red().white().bold()
    } else if snapshot.cpu_usage > 50.0 || snapshot.mem_usage > 75.0 {
        "Minor strain detected".yellow().bold()
    } else {
        "System stable".green().bold()
    };

    println!("\n{} {}", "Status:".bold(), status_line);
    println!("{} {}", "Advice:".bold(), interpretation.italic());

    Ok(())
}
