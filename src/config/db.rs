use std::fs::File;
use std::path::Path;
use sqlx::sqlite::SqlitePool;
use tokio::fs;

pub type DbPool = SqlitePool;

pub async fn connect_db(database_url: impl AsRef<str>) -> Result<DbPool, sqlx::Error> {
    let database_url = database_url.as_ref();
    let db_path =database_url.trim_start_matches("sqlite://");

    // Checking the parent directory and if it doesn't exists, we will create it
    let parent = Path::new(db_path).parent();
    if let Some(dir) = parent {
        fs::create_dir_all(dir).await.ok();
    }

    // Checking the database file existence and if it doesn't exists, we will create it
    if !Path::new(db_path).exists() {
        File::create(db_path)
            .expect("Failed to created file");
    };

    SqlitePool::connect(&database_url).await
}
