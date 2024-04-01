use crate::{custom::HtmlResponse, user};
use axum::{
    extract::Query,
    http::HeaderMap,
    response::{IntoResponse, Redirect},
};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{utils::env::ENV, view::render_template};

use super::service;

#[derive(Deserialize)]
pub struct AuthCallbackQueryControllerParams {
    code: String,
}

fn generate_github_url() -> String {
    let client_id = ENV.github_client_id();
    let redirect_uri = ENV.github_redirect_uri();
    let scope = ENV.github_scope();

    format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}",
        client_id, redirect_uri, scope
    )
}

pub async fn login_html() -> impl IntoResponse {
    let mut params = HashMap::new();
    params.insert("url", generate_github_url());

    render_template("login", params.into()).to_html_response()
}

pub async fn auth_callback(
    Query(query): Query<AuthCallbackQueryControllerParams>,
) -> impl IntoResponse {
    let access_token = match service::get_github_access_token(&query.code).await {
        Err(err) => return (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        Ok(access_token) => access_token,
    };

    let github_user = match service::get_github_user(&access_token).await {
        Err(_) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Unable to retrieve github user",
            )
                .into_response()
        }
        Ok(u) => u,
    };

    match user::service::save_user(github_user.into()).await {
        Err(_) => {
            return (StatusCode::UNPROCESSABLE_ENTITY, "Unable to save the user").into_response()
        }
        Ok(_) => (),
    };

    let access_token_cookie = format!("access_token={}; SameSite=Lax", access_token);
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        access_token_cookie.parse().unwrap(),
    );

    (headers, Redirect::to("/me")).into_response()
}
