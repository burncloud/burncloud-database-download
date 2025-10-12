//! Real database example for burncloud-database-download
//!
//! This example demonstrates using a real SQLite file database instead of an in-memory one.
//! The database file will be created at ./test_downloads.db

use burncloud_database_download::{DownloadRepository, DownloadTask, DownloadProgress};
use burncloud_database::Database;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== burncloud-database-download Real Database Example ===\n");

    println!("1. Creating database (using default path)...");

    // Create a database using the new API (uses default path)
    let db = Database::new().await?;

    // 2. Create repository and initialize schema
    println!("2. Initializing download repository...");
    let repo = DownloadRepository::new(db);
    repo.initialize().await?;
    println!("   ✓ Schema initialized\n");

    // 3. Create and save real download tasks
    println!("3. Creating download tasks with real data...");

    let task1 = DownloadTask::new(
        "https://releases.ubuntu.com/22.04/ubuntu-22.04.3-desktop-amd64.iso".to_string(),
        PathBuf::from("C:/Downloads/ubuntu-22.04.3-desktop-amd64.iso")
    );

    let task2 = DownloadTask::new(
        "https://download.mozilla.org/?product=firefox-latest&os=win64&lang=en-US".to_string(),
        PathBuf::from("C:/Downloads/firefox-installer.exe")
    );

    let task3 = DownloadTask::new(
        "https://code.visualstudio.com/sha/download?build=stable&os=win32-x64".to_string(),
        PathBuf::from("C:/Downloads/vscode-installer.exe")
    );

    repo.save_task(&task1).await?;
    repo.save_task(&task2).await?;
    repo.save_task(&task3).await?;
    println!("   ✓ Saved 3 tasks\n");

    // 4. Simulate download progress for each task
    println!("4. Saving realistic download progress...");

    // Ubuntu ISO - 4.7GB file, 50% downloaded
    let progress1 = DownloadProgress {
        downloaded_bytes: 2_500_000_000, // 2.5GB
        total_bytes: Some(5_000_000_000), // 5GB
        speed_bps: 10_485_760, // 10 MB/s
        eta_seconds: Some(238), // ~4 minutes
    };
    repo.save_progress(&task1.id, &progress1).await?;

    // Firefox - 100MB file, 75% downloaded
    let progress2 = DownloadProgress {
        downloaded_bytes: 75_000_000, // 75MB
        total_bytes: Some(100_000_000), // 100MB
        speed_bps: 5_242_880, // 5 MB/s
        eta_seconds: Some(5),
    };
    repo.save_progress(&task2.id, &progress2).await?;

    // VSCode - 150MB file, 25% downloaded
    let progress3 = DownloadProgress {
        downloaded_bytes: 37_500_000, // 37.5MB
        total_bytes: Some(150_000_000), // 150MB
        speed_bps: 8_388_608, // 8 MB/s
        eta_seconds: Some(13),
    };
    repo.save_progress(&task3.id, &progress3).await?;

    println!("   ✓ Progress saved for all tasks\n");

    // 5. Retrieve and display all tasks with progress
    println!("5. Retrieving saved data from database...");
    let tasks = repo.list_tasks().await?;
    println!("   Found {} tasks:\n", tasks.len());

    for (idx, task) in tasks.iter().enumerate() {
        println!("   Task {}:", idx + 1);
        println!("     URL: {}", task.url);
        println!("     Target: {}", task.target_path.display());
        println!("     Status: {}", task.status);

        if let Ok(progress) = repo.get_progress(&task.id).await {
            if let Some(percentage) = progress.completion_percentage() {
                let downloaded_mb = progress.downloaded_bytes as f64 / 1_048_576.0;
                let total_mb = progress.total_bytes.unwrap_or(0) as f64 / 1_048_576.0;
                let speed_mbps = progress.speed_bps as f64 / 1_048_576.0;

                println!("     Progress: {:.1}% ({:.1}MB / {:.1}MB)",
                    percentage, downloaded_mb, total_mb);
                println!("     Speed: {:.2} MB/s", speed_mbps);

                if let Some(eta) = progress.eta_seconds {
                    let minutes = eta / 60;
                    let seconds = eta % 60;
                    println!("     ETA: {}m {}s", minutes, seconds);
                }
            }
        }
        println!();
    }

    // 6. Statistics
    println!("6. Database statistics:");
    let total_count = repo.count_tasks().await?;
    println!("   Total tasks in database: {}", total_count);

    let status_counts = repo.count_tasks_by_status().await?;
    println!("   Tasks by status:");
    for (status, count) in status_counts {
        println!("     - {}: {}", status, count);
    }
    println!();

    println!("7. Database persisted to default location");
    println!("   Note: The new API uses a default database path");
    println!("   You can query tasks with: SELECT * FROM download_tasks;\n");

    println!("=== Example completed successfully! ===");
    println!("Note: The database file has been kept for inspection.\n");

    Ok(())
}
