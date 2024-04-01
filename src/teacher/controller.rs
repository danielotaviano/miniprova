use axum::{response::IntoResponse, Extension};
use reqwest::StatusCode;

use crate::{class, custom::HtmlResponse, middleware::auth::AuthState, view::render_template};

pub async fn home_html(Extension(current_user): Extension<AuthState>) -> impl IntoResponse {
    match class::service::get_by_user_id(&current_user.get_user_id()).await {
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting the classes",
        )
            .into_response(),
        Ok(classes) => render_template("teacher/home", classes.into()).to_html_response(),
    }
}
