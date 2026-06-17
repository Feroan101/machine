use anyhow::Result;
use crate::storage::db::StorageManager;
use colored::*;

pub async fn run(json: bool, _verbose: bool) -> Result<()> {
    let storage = StorageManager::new().await?;
    let snapshots = storage.list_snapshots(20).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&snapshots)?);
        return Ok(());
    }

    println!("\n{}", " SYSTEM HISTORY ".on_blue().black().bold());
    
    if snapshots.is_empty() {
        println!("No snapshots recorded yet. Run '{}' to create one.", "machine snapshot".bold());
        return Ok(());
    }

    println!("{:<25} {:<20}", "SNAPSHOT ID".bold(), "RECORDED AT".bold());
    println!("{}", "-".repeat(45).bright_black());

    for (id, timestamp) in snapshots {
        println!("{:<25} {:<20}", id.cyan(), timestamp);
    }

    println!("\nUse '{}' to compare historical states.", "machine diff <id1> <id2>".bold());

    Ok(())
}
