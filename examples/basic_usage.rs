//! Basic usage example for burncloud-database-download
//!
//! This example demonstrates how to use the download repository to persist
//! download tasks and their progress to a SQLite database.

use burncloud_database_download::{DownloadRepository, DownloadTask, DownloadProgress};
use burncloud_database_core::create_in_memory_database;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== burncloud-database-download Basic Usage Example ===\n");

    // 1. Create an in-memory database for this example
    println!("1. Creating in-memory database...");
    let db = create_in_memory_database().await?;

    // 2. Create repository and initialize schema
    println!("2. Initializing download repository...");
    let repo = DownloadRepository::new(db);
    repo.initialize().await?;
    println!("   ✓ Schema initialized\n");

    // 3. Create and save download tasks
    println!("3. Creating download tasks...");
    let task1 = DownloadTask::new(
        "https://example.com/large-file.zip".to_string(),
        PathBuf::from("/downloads/large-file.zip")
    );
    let task2 = DownloadTask::new(
        "https://example.com/document.pdf".to_string(),
        PathBuf::from("/downloads/document.pdf")
    );

    repo.save_task(&task1).await?;
    repo.save_task(&task2).await?;
    println!("   ✓ Saved 2 tasks\n");

    // 4. Save download progress
    println!("4. Saving download progress...");
    let progress1 = DownloadProgress {
        downloaded_bytes: 5120,
        total_bytes: Some(10240),
        speed_bps: 1024,
        eta_seconds: Some(5),
    };
    repo.save_progress(&task1.id, &progress1).await?;

    let progress2 = DownloadProgress {
        downloaded_bytes: 2048,
        total_bytes: Some(4096),
        speed_bps: 512,
        eta_seconds: Some(4),
    };
    repo.save_progress(&task2.id, &progress2).await?;
    println!("   ✓ Progress saved for both tasks\n");

    // 5. Retrieve tasks and progress
    println!("5. Retrieving saved data...");
    let tasks = repo.list_tasks().await?;
    println!("   Found {} tasks:", tasks.len());
    for task in &tasks {
        println!("     - {} ({})", task.url, task.status);

        if let Ok(progress) = repo.get_progress(&task.id).await {
            if let Some(percentage) = progress.completion_percentage() {
                println!("       Progress: {:.1}% ({}/{} bytes)",
                    percentage,
                    progress.downloaded_bytes,
                    progress.total_bytes.unwrap_or(0)
                );
            }
        }
    }
    println!();

    // 6. Statistics
    println!("6. Database statistics:");
    let total_count = repo.count_tasks().await?;
    println!("   Total tasks: {}", total_count);

    let status_counts = repo.count_tasks_by_status().await?;
    println!("   Tasks by status:");
    for (status, count) in status_counts {
        println!("     - {}: {}", status, count);
    }
    println!();

    // 7. Delete a task
    println!("7. Deleting task 1...");
    repo.delete_task(&task1.id).await?;
    let remaining = repo.count_tasks().await?;
    println!("   ✓ Deleted. Remaining tasks: {}\n", remaining);

    // 8. Cleanup
    println!("8. Cleaning up all data...");
    repo.clear_all().await?;
    let final_count = repo.count_tasks().await?;
    println!("   ✓ All data cleared. Final count: {}\n", final_count);

    println!("=== Example completed successfully! ===");

    Ok(())
}