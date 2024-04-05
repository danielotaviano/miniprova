mod auth;
mod class;
mod custom;
mod exam;
mod infra;
mod middleware;
mod student;
mod teacher;
mod user;
mod utils;
mod view;

use axum::{
    routing::{get, post},
    Error, Router,
};
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
        .route("/teacher", get(teacher::controller::home_html))
        .route("/teacher/class", post(class::controller::create_class))
        .route(
            "/teacher/class/:class_id/create-exam",
            get(exam::controller::create_html).post(exam::controller::create),
        )
        .route("/student", get(student::controller::home_html))
        .route("/student/class/enroll", post(class::controller::enroll))
        .layer(axum::middleware::from_fn(auth_middleware))
        .route("/login", get(auth::controller::login_html))
        .route("/oauth", get(auth::controller::auth_callback));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
