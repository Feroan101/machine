use anyhow::Result;
use crate::core::analysis::Analyzer;
use crate::core::recommendations;
use colored::*;

pub async fn run(verbose: bool, json: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    let snapshot = analyzer.get_snapshot();
    
    println!("\n{}", " SYSTEM INVESTIGATION ".on_yellow().black().bold());
    println!("Running deep-dive checks on CPU, Memory, and System State...");

    let issues = recommendations::get_recommendations(snapshot.cpu_usage, snapshot.mem_usage);

    // Simulated service check
    if verbose {
        println!("Checking essential services...");
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&issues)?);
        return Ok(());
    }

    if issues.is_empty() {
        println!("\n{}", "No critical anomalies detected.".green());
    } else {
        for issue in issues {
            recommendations::print_issue(&issue);
        }
    }

    Ok(())
}
