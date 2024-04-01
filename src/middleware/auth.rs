use crate::auth;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use reqwest::StatusCode;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AuthState {
    user_id: String,
}

impl AuthState {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
        }
    }

    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }
}

pub async fn auth_middleware(mut request: Request, next: Next) -> impl IntoResponse {
    let cookies = request.headers().get("Cookie");

    let access_token = cookies.and_then(|c| {
        let cookies = c.to_str().unwrap_or("");
        let cookie_map: HashMap<&str, &str> = cookies
            .split(';')
            .map(|s| {
                let mut parts = s.splitn(2, '=');
                (
                    parts.next().unwrap().trim(),
                    parts.next().unwrap_or("").trim(),
                )
            })
            .collect();
        cookie_map.get("access_token").cloned()
    });

    let access_token = match access_token {
        None => return Redirect::to("/login").into_response(),
        Some(at) => at,
    };

    let user = match auth::service::get_github_user(&access_token).await {
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
        Ok(user) => user,
    };

    request
        .extensions_mut()
        .insert(AuthState::new(user.get_id()));

    next.run(request).await
}
