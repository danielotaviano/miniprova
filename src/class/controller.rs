use axum::{
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_extra::extract::Form;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AuthState;

use super::{model::Class, service};

#[derive(Deserialize, Serialize)]
pub struct CreateClassControllerParams {
    code: String,
    name: String,
    description: String,
}

pub async fn create_class(
    Extension(current_user): Extension<AuthState>,
    Form(params): Form<CreateClassControllerParams>,
) -> impl IntoResponse {
    let class = Class::new(
        &params.code,
        &params.name,
        &params.description,
        current_user.get_user_id(),
    );

    match service::create(class).await {
        Err(_) => (
            StatusCode::BAD_REQUEST,
            "Error when trying to create a new class",
        )
            .into_response(),
        Ok(_) => Redirect::to("/teacher").into_response(),
    }
}
