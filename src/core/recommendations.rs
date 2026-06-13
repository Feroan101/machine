use serde::Serialize;
use colored::*;

#[derive(Debug, Serialize)]
pub struct Issue {
    pub title: String,
    pub severity: Severity,
    pub cause: String,
    pub recommendation: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

pub fn get_recommendations(cpu: f32, mem: f32) -> Vec<Issue> {
    let mut issues = Vec::new();

    if cpu > 80.0 {
        issues.push(Issue {
            title: "High CPU Usage".to_string(),
            severity: Severity::High,
            cause: "One or more processes are placing a heavy load on the processor.".to_string(),
            recommendation: "Identify the top CPU consumer and consider closing it if not needed.".to_string(),
        });
    }

    if mem > 90.0 {
        issues.push(Issue {
            title: "Memory Pressure".to_string(),
            severity: Severity::Critical,
            cause: "Almost all available RAM is in use. This can lead to system freezes.".to_string(),
            recommendation: "Close unused browser tabs or memory-heavy applications. Check for 'zombie' processes.".to_string(),
        });
    } else if mem > 75.0 {
        issues.push(Issue {
            title: "Elevated Memory Usage".to_string(),
            severity: Severity::Medium,
            cause: "System is using a significant portion of available RAM.".to_string(),
            recommendation: "Monitor memory usage. If it continues to rise, identify the culprit.".to_string(),
        });
    }

    issues
}

pub fn print_issue(issue: &Issue) {
    let severity_str = match issue.severity {
        Severity::Critical => "CRITICAL".on_red().white().bold(),
        Severity::High => "HIGH".red().bold(),
        Severity::Medium => "MEDIUM".yellow().bold(),
        Severity::Low => "LOW".green().bold(),
    };

    println!("\n{} {}", "Issue:".bold(), issue.title);
    println!("{} {}", "Severity:".bold(), severity_str);
    println!("{} {}", "Cause:".bold(), issue.cause);
    println!("{} {}", "Recommendation:".bold(), issue.recommendation.bright_blue());
}
