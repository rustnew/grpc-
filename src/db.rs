use sqlx::{Pool, Postgres, Row};
use chrono::{DateTime, Utc};
use std::env;
use dotenv::dotenv;

#[derive(sqlx::FromRow,  Debug)]
pub struct Command {
    pub id: i32,
    pub command: String,
    pub output: String,
    pub created_at: DateTime<Utc>,
}

pub async fn init_db_pool() -> Result<Pool<Postgres>, sqlx::Error> {

    let   database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    });

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn save_command(pool: &Pool<Postgres>, command: &str, output: &str) -> Result<i32, sqlx::Error> {

    let result = sqlx::query("INSERT INTO commands (command, output) VALUES ($1, $2) RETURNING id")
        .bind(command)
        .bind(output)
        .fetch_one(pool)
        .await?;
    Ok(result.get("id"))
}

pub async fn get_command(pool: &Pool<Postgres>, id: i32) -> Result<Option<Command>, sqlx::Error> {
    sqlx::query_as::<_, Command>("SELECT * FROM commands WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn list_commands(pool: &Pool<Postgres>) -> Result<Vec<Command>, sqlx::Error> {
    sqlx::query_as::<_, Command>("SELECT * FROM commands ORDER BY created_at")
        .fetch_all(pool)
        .await
}