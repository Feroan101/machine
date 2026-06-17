use anyhow::Result;
use colored::*;
use crate::core::analysis::Analyzer;
use crate::ui::format;

pub async fn run(json: bool, _verbose: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    let snapshot = analyzer.get_snapshot();

    if json {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
        return Ok(());
    }

    println!("\n{}", " MEMORY ANALYSIS ".on_magenta().white().bold());
    
    let total_mem = analyzer.sys.total_memory();
    let used_mem = analyzer.sys.used_memory();
    let free_mem = total_mem - used_mem;
    
    println!("{:<15} {}", "Total RAM:".bold(), format::format_bytes(total_mem));
    println!("{:<15} {}", "Used RAM:".bold(), format::format_bytes(used_mem).red());
    println!("{:<15} {}", "Free RAM:".bold(), format::format_bytes(free_mem).green());
    println!("{:<15} {:.1}%", "RAM Usage:".bold(), snapshot.mem_usage);

    println!("\n{}", "--- Swap Space ---".bright_black());
    let total_swap = analyzer.sys.total_swap();
    let used_swap = analyzer.sys.used_swap();
    println!("{:<15} {}", "Total Swap:".bold(), format::format_bytes(total_swap));
    println!("{:<15} {}", "Used Swap:".bold(), format::format_bytes(used_swap));
    println!("{:<15} {:.1}%", "Swap Usage:".bold(), snapshot.swap_usage);

    if snapshot.mem_usage > 90.0 {
        println!("\n{} Memory pressure is high. Consider closing heavy applications.", "CAUTION:".red().bold());
    }

    Ok(())
}
