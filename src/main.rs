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
    response::Redirect,
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
        .route(
            "/teacher/exam/:exam_id/results",
            get(teacher::controller::exam_results_html),
        )
        .route(
            "/teacher/class/:class_id/exams",
            get(class::controller::list_exams),
        )
        .route("/student", get(student::controller::home_html))
        .route("/student/class/enroll", post(class::controller::enroll))
        .route(
            "/student/exam/:exam_id",
            get(student::controller::exam_html),
        )
        .route(
            "/student/exam/:exam_id/result",
            get(student::controller::exam_result_html),
        )
        .route(
            "/student/exam/:exam_id/:question_id/save-answer",
            post(student::controller::save_answer),
        )
        .layer(axum::middleware::from_fn(auth_middleware))
        .route("/login", get(auth::controller::login_html))
        .route("/oauth", get(auth::controller::auth_callback))
        .route("/", get(Redirect::to("/me")));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
