use anyhow::Result;
use crate::{ProcessCommands, core::analysis::Analyzer, core::proc};
use colored::*;
use crate::ui::format;

pub async fn run(sub: Option<ProcessCommands>, json: bool, _verbose: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    analyzer.get_snapshot();

    match sub {
        Some(ProcessCommands::Top) => {
            let top_procs = proc::get_top_processes(&analyzer.sys, 10);
            if json {
                println!("{}", serde_json::to_string_pretty(&top_procs)?);
                return Ok(());
            }
            println!("\n{}", " TOP RESOURCE CONSUMERS ".on_blue().white().bold());
            println!("{:<8} {:<20} {:<10} {:<15}", "PID".bold(), "Name".bold(), "CPU %".bold(), "Memory".bold());
            println!("{}", "-".repeat(55).bright_black());
            for p in top_procs {
                println!("{:<8} {:<20} {:>5.1}% {:>15}", p.pid, p.name, p.cpu_usage, format::format_bytes(p.mem_usage));
            }
        },
        Some(ProcessCommands::Zombie) => {
            let zombies: Vec<_> = analyzer.sys.processes().values()
                .filter(|p| p.status().to_string().contains("Zombie") || p.status().to_string().contains("Defunct"))
                .collect();
            
            if json {
                let zombie_info: Vec<_> = zombies.iter().map(|z| serde_json::json!({"pid": z.pid().as_u32(), "name": z.name()})).collect();
                println!("{}", serde_json::to_string_pretty(&zombie_info)?);
                return Ok(());
            }

            println!("\n{}", " ZOMBIE PROCESSES ".on_red().white().bold());
            if zombies.is_empty() {
                println!("{}", "No zombie processes found.".green());
            } else {
                for z in zombies {
                    println!("  • {} (PID: {})", z.name(), z.pid());
                }
            }
        },
        Some(ProcessCommands::Heavy) => {
            let mut processes: Vec<_> = analyzer.sys.processes().values().collect();
            processes.sort_by(|a, b| b.memory().cmp(&a.memory()));
            let top_heavy = processes.iter().take(10).cloned().collect::<Vec<_>>();

            if json {
                let heavy_info: Vec<_> = top_heavy.iter().map(|p| serde_json::json!({"pid": p.pid().as_u32(), "name": p.name(), "memory": p.memory()})).collect();
                println!("{}", serde_json::to_string_pretty(&heavy_info)?);
                return Ok(());
            }

            println!("\n{}", " HEAVY PROCESSES (Memory) ".on_magenta().white().bold());
            println!("{:<8} {:<20} {:<15}", "PID".bold(), "Name".bold(), "Memory".bold());
            println!("{}", "-".repeat(45).bright_black());
            for p in top_heavy {
                println!("{:<8} {:<20} {:>15}", p.pid(), p.name(), format::format_bytes(p.memory()));
            }
        },
        None => {
            println!("Please specify a subcommand for 'process' (top, zombie, heavy).");
        }
    }

    Ok(())
}
