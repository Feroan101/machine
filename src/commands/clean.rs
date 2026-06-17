use anyhow::Result;
use colored::*;
use std::fs;
use crate::ui::format;

pub async fn run(json: bool, _verbose: bool) -> Result<()> {
    println!("\n{}", " STORAGE RECLAMATION ".on_green().black().bold());
    println!("Scanning for non-essential files...\n");

    let mut total_reclaimable = 0u64;
    let mut details = serde_json::Map::new();

    // 1. /tmp analysis
    if let Ok(entries) = fs::read_dir("/tmp") {
        let mut tmp_size = 0u64;
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                tmp_size += metadata.len();
            }
        }
        if !json {
            println!("{:<20} {}", "Temp Files (/tmp):".bold(), format::format_bytes(tmp_size));
        }
        details.insert("tmp".to_string(), serde_json::json!(tmp_size));
        total_reclaimable += tmp_size;
    }

    // 2. Package cache (apt)
    let apt_cache = "/var/cache/apt/archives";
    if let Ok(entries) = fs::read_dir(apt_cache) {
        let mut cache_size = 0u64;
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                cache_size += metadata.len();
            }
        }
        println!("{:<20} {}", "Apt Cache:".bold(), format::format_bytes(cache_size));
        total_reclaimable += cache_size;
    }

    // 3. User cache
    if let Ok(home) = std::env::var("HOME") {
        let user_cache = format!("{}/.cache", home);
        if let Ok(entries) = fs::read_dir(&user_cache) {
            let mut u_cache_size = 0u64;
            // Don't recurse too deep for speed, just top level
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    u_cache_size += metadata.len();
                }
            }
            println!("{:<20} {}", "User Cache:".bold(), format::format_bytes(u_cache_size));
            total_reclaimable += u_cache_size;
        }
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&details)?);
        return Ok(());
    }

    println!("\n{}", "-".repeat(40).bright_black());
    println!("{:<20} {}", "TOTAL POTENTIAL:".bold(), format::format_bytes(total_reclaimable).green());

    println!("\n{} To reclaim this space, you might run:", "ADVICE:".blue().bold());
    println!("  • sudo apt-get clean");
    println!("  • rm -rf ~/.cache/* (be careful!)");
    println!("  • sudo journalctl --vacuum-time=3d");

    Ok(())
}
