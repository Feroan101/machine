use anyhow::Result;
use crate::core::analysis::Analyzer;
use crate::core::recommendations;
use colored::*;
use std::process::Command;
use sysinfo::Disks;

pub async fn run(verbose: bool, json: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    let snapshot = analyzer.get_snapshot();
    
    println!("\n{}", " SYSTEM INVESTIGATION ".on_yellow().black().bold());
    println!("Running deep-dive checks on CPU, Memory, Disk, and Network...");

    let mut issues = recommendations::get_recommendations(snapshot.cpu_usage, snapshot.mem_usage);

    // 1. Disk Space Check
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let usage = 100.0 - (disk.available_space() as f32 / disk.total_space() as f32 * 100.0);
        if usage > 90.0 {
            issues.push(recommendations::Issue {
                title: "Low Disk Space".to_string(),
                severity: recommendations::Severity::High,
                cause: format!("Disk {} is {:.1}% full.", disk.mount_point().display(), usage),
                recommendation: "Clean up temporary files or logs.".to_string(),
            });
        }
    }

    // 2. DNS/Network Check (Simple latency check)
    if verbose {
        println!("Checking network connectivity...");
    }
    let dns_start = std::time::Instant::now();
    let dns_ok = Command::new("host").arg("google.com").output().is_ok();
    let dns_duration = dns_start.elapsed();

    if !dns_ok {
        issues.push(recommendations::Issue {
            title: "Network connectivity issues".to_string(),
            severity: recommendations::Severity::Medium,
            cause: "DNS resolution failed.".to_string(),
            recommendation: "Check your internet connection or DNS settings.".to_string(),
        });
    } else if dns_duration.as_millis() > 500 {
        issues.push(recommendations::Issue {
            title: "High DNS Latency".to_string(),
            severity: recommendations::Severity::Low,
            cause: format!("DNS resolution took {}ms.", dns_duration.as_millis()),
            recommendation: "Consider using a faster DNS provider.".to_string(),
        });
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&issues)?);
        return Ok(());
    }

    if issues.is_empty() {
        println!("\n{}", "No critical anomalies detected. System is operating within normal bounds.".green());
    } else {
        println!("\nFound {} areas requiring attention:", issues.len());
        for issue in issues {
            recommendations::print_issue(&issue);
        }
    }

    Ok(())
}
