use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn start_connection() -> Result<(), sqlx::Error> {
    let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set.");
    let postgres_password =
        std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set.");
    let postgres_db = std::env::var("POSTGRES_DATABASE").expect("POSTGRES_DATABASE must be set.");
    let postgres_port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set.");
    let postgres_host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set.");

    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    let result = DB_POOL.set(pool);

    match result {
        Err(_) => panic!("Unable connect to database"),
        Ok(_) => {
            println!("Database Connected");
            Ok(())
        }
    }
}

pub fn get_database() -> &'static Pool<Postgres> {
    DB_POOL.get().expect("Error when trying to get database")
}
