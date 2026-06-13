use anyhow::Result;
use crate::core::analysis::Analyzer;
use crate::core::proc;
use colored::*;

pub async fn run(target: String, json: bool, _verbose: bool) -> Result<()> {
    let mut analyzer = Analyzer::new();
    analyzer.get_snapshot();

    let info = proc::get_process_by_target(&analyzer.sys, &target);

    if let Some(p) = info {
        let ancestry = proc::get_ancestry(&analyzer.sys, p.pid);

        if json {
            println!("{}", serde_json::to_string_pretty(&ancestry)?);
            return Ok(());
        }

        println!("\n{} {}", " TRACING ANCESTRY ".on_magenta().white().bold(), p.name.bold());
        
        for (i, (apid, aname)) in ancestry.iter().enumerate() {
            let indent = "    ".repeat(i.saturating_sub(1));
            let line = if i == 0 {
                format!("{}{}", indent, aname.bold())
            } else {
                format!("{}{}{}", indent, "└── ".bright_black(), aname.bold())
            };

            if *apid == p.pid {
                println!("{} (PID: {}) {}", line, apid, "<-- target".cyan());
            } else {
                println!("{} (PID: {})", line, apid);
            }
        }

    } else {
        println!("{} Could not find process matching '{}'", "Error:".red().bold(), target);
    }

    Ok(())
}
