use anyhow::Result;
use crate::core::analysis::Analyzer;
use crate::core::recommendations;
use colored::*;

pub async fn run(json: bool, verbose: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    let snapshot = analyzer.get_snapshot();
    let issues = recommendations::get_recommendations(snapshot.cpu_usage, snapshot.mem_usage);

    if json {
        println!("{}", serde_json::to_string_pretty(&issues)?);
        return Ok(());
    }

    println!("\n{}", " WHAT THE... DIAGNOSIS ".on_red().white().bold());

    if verbose {
        println!("{:<15} {:.1}%", "CPU Usage:".bold(), snapshot.cpu_usage);
        println!("{:<15} {:.1}%", "RAM Usage:".bold(), snapshot.mem_usage);
    }

    if issues.is_empty() {
        println!("\n{}", "Everything looks normal! I couldn't find any immediate problems.".green());
        println!("Try running with {} if you suspect something is wrong.", "--verbose".bold());
    } else {
        println!("\nI found {} potential issues that might be affecting your system:", issues.len());
        for issue in issues {
            recommendations::print_issue(&issue);
        }
    }

    Ok(())
}
