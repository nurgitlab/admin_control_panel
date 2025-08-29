use configs::config;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use std::fs;
use std::path::Path;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    config::Config::init().expect("Failed to initialize config");

    // Create DB pool
    let database_url = config::Config::global().database_url.clone();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS schema_migrations (version BIGINT PRIMARY KEY, applied_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
        .execute(&pool)
        .await?;

    let applied_migrations: Vec<i64> = sqlx::query_scalar(
        "SELECT version FROM schema_migrations ORDER BY version;",
    )
    .fetch_all(&pool)
    .await?;

    println!("Applied migrations: {applied_migrations:?}");

    let migrations_dir = Path::new("./migrations");
    let mut migration_dirs = fs::read_dir(migrations_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                entry.file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    migration_dirs.sort();

    for migration_dir in migration_dirs {
        let version = match migration_dir.split('_').next() {
            Some(v) => v.parse::<i64>().ok(),
            None => {
                println!("Skipping invalid migration dir: {migration_dir}");
                continue;
            }
        };

        if let Some(version) = version {
            if !applied_migrations.contains(&version) {
                println!("Applying migration: {migration_dir}");

                let up_path =
                    migrations_dir.join(&migration_dir).join("up.sql");
                if !up_path.exists() {
                    println!("Error: up.sql not found in {migration_dir}");
                    continue;
                }

                let up_sql = fs::read_to_string(&up_path)?;
                println!("Executing SQL:\n{up_sql}");

                let mut tx = pool.begin().await.map_err(|e| {
                    println!("Failed to begin transaction: {e}");
                    e
                })?;

                // Разделяем SQL на отдельные команды
                let commands: Vec<&str> = up_sql
                    .split(';')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                let mut success = true;
                for command in commands {
                    if command.is_empty() {
                        continue;
                    }

                    println!("Executing command: {}", command);
                    match sqlx::query(command).execute(&mut *tx).await {
                        Ok(_) => {
                            println!("✓ Command executed successfully");
                        }
                        Err(e) => {
                            println!("✗ Failed to execute command: {}", e);
                            success = false;
                            break;
                        }
                    }
                }

                if success {
                    match sqlx::query(
                        "INSERT INTO schema_migrations (version) VALUES ($1)",
                    )
                    .bind(version)
                    .execute(&mut *tx)
                    .await
                    {
                        Ok(_) => {
                            tx.commit().await.map_err(|e| {
                                println!("Failed to commit transaction: {e}");
                                e
                            })?;
                            println!(
                                "Migration {version} applied successfully"
                            );
                        }
                        Err(e) => {
                            println!("Failed to record migration: {e}");
                            tx.rollback().await.ok();
                            return Err(e.into());
                        }
                    }
                } else {
                    println!("Some commands failed, rolling back migration");
                    tx.rollback().await.ok();
                    return Err(
                        "Migration failed due to SQL command errors".into()
                    );
                }
            }
        }
    }

    Ok(())
}
