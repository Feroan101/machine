use anyhow::Result;
use colored::*;
use sysinfo::Disks;
use crate::ui::format;

pub async fn run(json: bool, _verbose: bool) -> Result<()> {
    let disks = Disks::new_with_refreshed_list();

    if json {
        let mut results = Vec::new();
        for disk in &disks {
            results.push(serde_json::json!({
                "mount": disk.mount_point(),
                "total": disk.total_space(),
                "used": disk.total_space() - disk.available_space(),
                "available": disk.available_space(),
            }));
        }
        println!("{}", serde_json::to_string_pretty(&results)?);
        return Ok(());
    }

    println!("\n{}", " STORAGE ANALYSIS ".on_cyan().black().bold());
    println!("{:<20} {:<10} {:<10} {:<10} {:<15}", "Mount".bold(), "Total".bold(), "Used".bold(), "Free".bold(), "Usage".bold());
    println!("{}", "-".repeat(65).bright_black());

    for disk in &disks {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total - available;
        let usage_pct = (used as f32 / total as f32) * 100.0;
        
        let bar_len = 10;
        let filled = (usage_pct / 100.0 * bar_len as f32) as usize;
        let bar = format!("[{}{}]", "#".repeat(filled), " ".repeat(bar_len - filled));
        
        let color = if usage_pct > 90.0 { "red" } else if usage_pct > 75.0 { "yellow" } else { "green" };

        println!("{:<20} {:<10} {:<10} {:<10} {:<15}", 
            disk.mount_point().display().to_string().cyan(),
            format::format_bytes(total),
            format::format_bytes(used),
            format::format_bytes(available),
            format!("{} {:.1}%", if color == "red" { bar.red() } else if color == "yellow" { bar.yellow() } else { bar.green() }, usage_pct)
        );
    }

    Ok(())
}
