use files_sdk::{FilesClient, automation::AutomationHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("FILES_API_KEY")?;
    let client = FilesClient::builder().api_key(&api_key).build()?;
    let handler = AutomationHandler::new(client);

    // Create automation for folder sync
    let automation = handler
        .create(
            "folder_sync",   // automation type
            Some("/sync/*"), // source
            None,            // destination
            None,            // destinations
            None,            // interval
            None,            // path
            None,            // trigger
        )
        .await?;

    println!("Created automation:");
    println!("  ID: {}", automation.id.unwrap_or(0));
    println!(
        "  Type: {}",
        automation.automation.as_deref().unwrap_or("N/A")
    );
    if let Some(path) = automation.source {
        println!("  Path: {}", path);
    }

    // List all automations
    let (automations, _pagination) = handler.list(None, Some(50), None).await?;
    println!("\nTotal automations: {}", automations.len());
    for auto in automations {
        println!(
            "  {} - {} ({})",
            auto.id.unwrap_or(0),
            auto.automation.as_deref().unwrap_or("N/A"),
            auto.source.as_deref().unwrap_or("N/A")
        );
    }

    // Get specific automation
    // let automation = handler.get(automation.id).await?;
    // println!("\nAutomation {} details:", automation.id);
    // println!("  Enabled: {}", automation.disabled.map(|d| !d).unwrap_or(true));

    // Delete automation (cleanup)
    // handler.delete(automation.id).await?;
    // println!("\nDeleted automation {}", automation.id);

    Ok(())
}
