//! Status command implementation

use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

use crate::config::Config;
use crate::state::SyncState;

pub async fn handle_status(path: Option<PathBuf>) -> Result<()> {
    let config = Config::load()?;

    if config.watch.is_empty() {
        println!("{}", "No watch configurations found".yellow());
        return Ok(());
    }

    // Filter by path if specified
    let watches = if let Some(ref p) = path {
        let p = p.canonicalize()?;
        config
            .watch
            .iter()
            .filter(|w| w.local_path == p)
            .collect::<Vec<_>>()
    } else {
        config.watch.iter().collect::<Vec<_>>()
    };

    if watches.is_empty() {
        if let Some(p) = path {
            anyhow::bail!("No watch configuration found for: {}", p.display());
        }
    }

    println!("{}", "Watch Status".bold());
    println!();

    for watch in watches {
        println!("{}", "─".repeat(60));
        println!("{}: {}", "Local".cyan(), watch.local_path.display());
        println!("{}: {}", "Remote".cyan(), watch.remote_path);
        println!("{}: {}", "Direction".cyan(), watch.direction);

        // Load state
        match SyncState::load(&watch.local_path) {
            Ok(state) => {
                println!("{}: {}", "Files synced".cyan(), state.file_count());

                if let Some(last_sync) = state.last_sync {
                    println!(
                        "{}: {}",
                        "Last sync".cyan(),
                        last_sync.format("%Y-%m-%d %H:%M:%S")
                    );
                } else {
                    println!("{}: Never", "Last sync".cyan());
                }
            }
            Err(e) => {
                println!("{}: Failed to load state - {}", "Error".red(), e);
            }
        }

        if !watch.ignore_patterns.is_empty() {
            println!(
                "{}: {}",
                "Ignoring".cyan(),
                watch.ignore_patterns.join(", ")
            );
        }

        println!();
    }

    Ok(())
}
