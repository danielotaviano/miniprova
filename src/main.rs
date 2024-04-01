mod auth;
mod custom;
mod infra;
mod middleware;
mod user;
mod utils;
mod view;

use axum::{routing::get, Error, Router};
use dotenv::dotenv;
use infra::db::start_connection;
use middleware::auth::auth_middleware;

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    start_connection()
        .await
        .expect("Error when trying to connect the database");

    let app = Router::new()
        .route("/me", get(user::controller::me_html))
        .layer(axum::middleware::from_fn(auth_middleware))
        .route("/login", get(auth::controller::login_html))
        .route("/oauth", get(auth::controller::auth_callback));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
