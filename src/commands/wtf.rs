use anyhow::Result;
use crate::core::analysis::Analyzer;
use crate::core::recommendations;
use colored::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fs;

static WTF_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn interpret_bf(code: &str) -> String {
    let code: Vec<char> = code.chars().collect();
    let mut cells = vec![0u8; 30000];
    let mut cell_ptr = 0;
    let mut code_ptr = 0;
    let mut output = String::new();
    
    while code_ptr < code.len() {
        match code[code_ptr] {
            '>' => cell_ptr += 1,
            '<' => if cell_ptr > 0 { cell_ptr -= 1 },
            '+' => cells[cell_ptr] = cells[cell_ptr].wrapping_add(1),
            '-' => cells[cell_ptr] = cells[cell_ptr].wrapping_sub(1),
            '.' => output.push(cells[cell_ptr] as char),
            '[' => if cells[cell_ptr] == 0 {
                let mut depth = 1;
                while depth > 0 {
                    code_ptr += 1;
                    if code[code_ptr] == '[' { depth += 1 }
                    else if code[code_ptr] == ']' { depth -= 1 }
                }
            },
            ']' => if cells[cell_ptr] != 0 {
                let mut depth = 1;
                while depth > 0 {
                    code_ptr -= 1;
                    if code[code_ptr] == ']' { depth += 1 }
                    else if code[code_ptr] == '[' { depth -= 1 }
                }
            },
            _ => {}
        }
        code_ptr += 1;
    }
    output
}

pub async fn run(json: bool, verbose: bool) -> Result<()> {
    let count = WTF_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;

    if count == 3 {
        println!("{}", "\nanomalous behavior detected...".red().italic());
        
        // Load and execute the secret instruction set
        if let Ok(bf_code) = fs::read_to_string("src/core/bf_cache.bf") {
            let result = interpret_bf(&bf_code);
            println!("{}", result.bold());
        } else {
            // Fallback if file is missing (though it shouldn't be)
            println!("{}", "you've asked too many existential questions. the machine is judging you.".bold());
        }
    }

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
