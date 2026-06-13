use anyhow::{Result, anyhow};
use crate::core::analysis::Analyzer;
use crate::core::proc::{get_process_by_target, get_ancestry};
use colored::*;

pub async fn run(target: String, json: bool) -> Result<()> {
    let analyzer = Analyzer::new();
    let process = get_process_by_target(&analyzer.sys, &target)
        .ok_or_else(|| anyhow!("Could not find process matching '{}'", target))?;

    if json {
        println!("{}", serde_json::to_string_pretty(&process)?);
        return Ok(());
    }

    println!("\n{}", format!(" WHY IS '{}' RUNNING? ", process.name.to_uppercase()).on_cyan().black().bold());

    let ancestry = get_ancestry(&analyzer.sys, process.pid);
    let parent = ancestry.get(ancestry.len().saturating_sub(2));

    let purpose = if let Some((_, p_name)) = parent {
        let p_name_low = p_name.to_lowercase();
        if p_name_low.contains("systemd") {
            "System service or daemon managed by systemd."
        } else if p_name_low.contains("bash") || p_name_low.contains("zsh") || p_name_low.contains("sh") || p_name_low.contains("fish") {
            "User-initiated command started from a terminal."
        } else if p_name_low.contains("firefox") || p_name_low.contains("chrome") || p_name_low.contains("browser") || p_name_low.contains("electron") {
            "Subset or component of a web browser or Electron application."
        } else if p_name_low.contains("gnome") || p_name_low.contains("kde") || p_name_low.contains("xfce") || p_name_low.contains("window") {
            "Part of your desktop environment's GUI infrastructure."
        } else {
            "General application process or subprocess."
        }
    } else {
        "Root system process or independent daemon."
    };

    println!("\n{} {}", "Purpose:".bold(), purpose);
    
    if let Some((p_vid, p_name)) = parent {
        println!("{} {} (PID: {})", "Parent:".bold(), p_name, p_vid);
    }

    println!("\n{} {:.1}%", "CPU Usage:".bold(), process.cpu_usage);
    println!("{} {:.1} MB", "Memory:".bold(), process.mem_usage as f32 / 1024.0 / 1024.0);

    let status = if process.cpu_usage > 50.0 {
        "Unusual - process is consuming significant CPU resources.".yellow().bold()
    } else if process.cpu_usage > 10.0 {
        "Expected - active workload.".green()
    } else {
        "Expected - idling or performing background tasks.".green()
    };

    println!("\n{} {}", "Judgment:".bold(), status);

    Ok(())
}
