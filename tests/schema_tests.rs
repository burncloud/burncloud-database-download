use burncloud_database_download::schema::*;

#[test]
fn test_sql_syntax() {
    // Basic syntax validation - these should not panic when parsing
    assert!(CREATE_DOWNLOAD_TASKS_TABLE.contains("CREATE TABLE"));
    assert!(CREATE_DOWNLOAD_PROGRESS_TABLE.contains("CREATE TABLE"));
    assert!(CREATE_INDEXES.contains("CREATE INDEX"));
}

#[tokio::test]
async fn test_schema_initialization() {
    use burncloud_database::Database;

    let db = Database::new().await.unwrap();
    let result = initialize_schema(&db).await;
    assert!(result.is_ok());
}