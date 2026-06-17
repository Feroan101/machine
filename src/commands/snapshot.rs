use anyhow::Result;
use crate::core::analysis::Analyzer;
use crate::storage::db::StorageManager;
use chrono::Local;
use colored::*;

pub async fn run(_json: bool, _verbose: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    let snapshot = analyzer.get_snapshot();
    let storage = StorageManager::new().await?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let id = format!("snap_{}", timestamp);
    let data = serde_json::to_string(&snapshot)?;

    storage.save_snapshot(&id, &data).await?;

    println!("\n{} Snapshot saved: {}", " SUCCESS ".on_green().black().bold(), id.cyan().bold());
    println!("Use '{}' to view recorded states.", "machine history".bold());

    Ok(())
}
