use crate::custom::HtmlResponse;
use axum::{
    response::{IntoResponse, Redirect},
    Extension,
};
use reqwest::StatusCode;

use crate::{middleware::auth::AuthState, view::render_template};

use super::service;

pub async fn me_html(Extension(current_user): Extension<AuthState>) -> impl IntoResponse {
    let user = match service::get_user(current_user.get_user_id()).await {
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(optional_user) => optional_user,
    };

    let user = match user {
        None => return (Redirect::to("/login")).into_response(),
        Some(u) => u,
    };

    render_template("user/me", user.into()).to_html_response()
}
