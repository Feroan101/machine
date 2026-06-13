use anyhow::Result;
use crate::core::analysis::Analyzer;
use crate::core::proc;
use crate::ui::format;
use colored::*;

pub async fn run(target: String, json: bool, verbose: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    analyzer.get_snapshot(); // Refresh
    
    let info = proc::get_process_by_target(&analyzer.sys, &target);

    if let Some(p) = info {
        if json {
            println!("{}", serde_json::to_string_pretty(&p)?);
            return Ok(());
        }

        println!("\n{} {} (PID: {})", " EXPLAINING ".on_blue().white().bold(), p.name.bold(), p.pid);
        
        println!("\n{}", "--- Behavior ---".bright_black());
        println!("{:<15} {:.1}%", "CPU Usage:".bold(), p.cpu_usage);
        println!("{:<15} {}", "Memory (RAM):".bold(), format::format_bytes(p.mem_usage));
        println!("{:<15} {}", "Active for:".bold(), format::format_duration(p.uptime));

        if verbose {
            println!("{:<15} {}", "Status:".bold(), "Running");
            // In a real production app, we would add open files/network here
        }

        println!("\n{}", "--- Relations ---".bright_black());
        if let Some((ppid, pname)) = proc::get_parent_process(&analyzer.sys, p.pid) {
            println!("{:<15} {} (PID: {})", "Started by:".bold(), pname, ppid);
        } else {
            println!("{:<15} None (System Process)", "Started by:".bold());
        }

        let children = proc::get_child_processes(&analyzer.sys, p.pid);
        if !children.is_empty() {
            println!("{:<15} {} subprocesses", "Managing:".bold(), children.len());
        }

        println!("\n{}", "--- Recommendation ---".bright_black());
        if p.cpu_usage > 50.0 {
            println!("{}", "This process is currently busy. If it's not a video editor or game, it might be stuck.".yellow());
        } else {
            println!("{}", "This process is behaving normally.".green());
        }

    } else {
        println!("{} Could not find process matching '{}'", "Error:".red().bold(), target);
    }

    Ok(())
}
