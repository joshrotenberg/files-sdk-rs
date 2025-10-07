//! List command implementation

use anyhow::Result;
use colored::Colorize;

use crate::config::Config;

pub async fn handle_list() -> Result<()> {
    let config = Config::load()?;

    if config.watch.is_empty() {
        println!("{}", "No watch configurations found".yellow());
        println!("\n{}", "Run 'files-watch init' to create one".cyan());
        return Ok(());
    }

    println!("{}", "Configured Watches".bold());
    println!();

    for (idx, watch) in config.watch.iter().enumerate() {
        println!(
            "{}. {}",
            idx + 1,
            watch.local_path.display().to_string().green()
        );
        println!("   Remote:    {}", watch.remote_path);
        println!("   Direction: {}", watch.direction);
        if !watch.ignore_patterns.is_empty() {
            println!("   Ignore:    {}", watch.ignore_patterns.join(", "));
        }
        println!();
    }

    Ok(())
}
