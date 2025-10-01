//! Verify the persisted database contents
//!
//! This example reads from the existing test_downloads.db file to verify
//! that data was correctly persisted.

use burncloud_database_download::{DownloadRepository};
use burncloud_database::create_database;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== Verifying Persisted Database ===\n");

    let db_path = PathBuf::from("./test_downloads.db");

    if !db_path.exists() {
        println!("❌ Error: Database file not found at {:?}", db_path);
        println!("   Please run 'cargo run --example real_database' first.\n");
        return Ok(());
    }

    println!("✓ Found database file at: {:?}\n", db_path);

    // Open the existing database
    let db = create_database(&db_path).await?;
    let repo = DownloadRepository::new(db);

    // Read all tasks
    println!("📊 Reading tasks from database...\n");
    let tasks = repo.list_tasks().await?;

    if tasks.is_empty() {
        println!("⚠️  No tasks found in database");
    } else {
        println!("Found {} tasks:\n", tasks.len());

        for (idx, task) in tasks.iter().enumerate() {
            println!("Task {}:", idx + 1);
            println!("  ID: {}", task.id);
            println!("  URL: {}", task.url);
            println!("  Target: {}", task.target_path.display());
            println!("  Status: {:?}", task.status);
            println!("  Created: {:?}", task.created_at);
            println!("  Updated: {:?}", task.updated_at);

            // Try to get progress
            if let Ok(progress) = repo.get_progress(&task.id).await {
                println!("  Progress:");
                println!("    Downloaded: {} bytes", progress.downloaded_bytes);
                println!("    Total: {:?} bytes", progress.total_bytes);
                println!("    Speed: {} bytes/sec", progress.speed_bps);
                println!("    ETA: {:?} seconds", progress.eta_seconds);

                if let Some(percentage) = progress.completion_percentage() {
                    println!("    Completion: {:.2}%", percentage);
                }
            } else {
                println!("  Progress: No data");
            }
            println!();
        }
    }

    // Statistics
    println!("📈 Statistics:");
    let total_count = repo.count_tasks().await?;
    println!("  Total tasks: {}", total_count);

    let status_counts = repo.count_tasks_by_status().await?;
    println!("  By status:");
    for (status, count) in status_counts {
        println!("    - {}: {}", status, count);
    }
    println!();

    println!("✓ Database verification complete!\n");

    Ok(())
}
