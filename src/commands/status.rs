use anyhow::Result;
use crate::core::analysis::Analyzer;
use crate::core::proc;
use crate::ui::colors;
use crate::ui::format;
use colored::*;

pub async fn run(json: bool, verbose: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    let snapshot = analyzer.get_snapshot();
    let summary = analyzer.explain_health(&snapshot);

    if json {
        println!("{}", serde_json::to_string_pretty(&snapshot)?);
        return Ok(());
    }

    println!("\n{}", " SYSTEM STATUS ".on_cyan().black().bold());
    println!("{}: {}", "Summary".bold(), summary);
    
    if verbose {
        println!("\n{}", "--- System Info ---".bright_black());
        println!("{:<12} {}", "Hostname:".bold(), snapshot.hostname);
        println!("{:<12} {}", "Kernel:".bold(), snapshot.kernel);
        println!("{:<12} {}", "OS:".bold(), snapshot.os);
    }

    println!("\n{}", "--- Resources ---".bright_black());
    print_resource("CPU", snapshot.cpu_usage);
    print_resource("RAM", snapshot.mem_usage);
    print_resource("Swap", snapshot.swap_usage);

    println!("\n{} {}", "Uptime:".bold(), format::format_duration(snapshot.uptime));
    println!("{} {:.2}, {:.2}, {:.2}", "Load Avg:".bold(), snapshot.load_avg.0, snapshot.load_avg.1, snapshot.load_avg.2);

    println!("\n{}", "--- Top Processes ---".bright_black());
    let top_procs = proc::get_top_processes(&analyzer.sys, if verbose { 10 } else { 5 });
    for p in top_procs {
        println!("{:<15} {:>6.1}% CPU  {:>10}", 
            if p.name.len() > 14 { format!("{}...", &p.name[0..11]) } else { p.name }, 
            p.cpu_usage, format::format_bytes(p.mem_usage));
    }

    Ok(())
}

fn print_resource(label: &str, usage: f32) {
    let color = colors::get_severity_color(usage);
    let bar_len = 20;
    let filled = (usage / 100.0 * bar_len as f32) as usize;
    let empty = bar_len - filled;
    
    let bar = format!(
        "[{}{}]",
        "|".repeat(filled),
        " ".repeat(empty)
    );

    println!("{:<6} {:>5.1}% {:<22}", label.bold(), usage, bar.color(color));
}
