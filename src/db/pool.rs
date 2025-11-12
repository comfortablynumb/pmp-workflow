use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool, Postgres};

/// Create a new database connection pool
pub async fn create_pool(database_url: &str) -> Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to connect to database")
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    // Read and execute migration files in order
    let migrations = [
        include_str!("../../migrations/001_create_workflows.sql"),
        include_str!("../../migrations/002_create_executions.sql"),
    ];

    for (idx, migration) in migrations.iter().enumerate() {
        tracing::info!("Running migration {}", idx + 1);
        sqlx::raw_sql(migration)
            .execute(pool)
            .await
            .with_context(|| format!("Failed to run migration {}", idx + 1))?;
    }

    tracing::info!("All migrations completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires a database connection
    async fn test_create_pool() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost/pmp_workflow_test".to_string()
        });

        let result = create_pool(&database_url).await;
        assert!(result.is_ok());
    }
}
