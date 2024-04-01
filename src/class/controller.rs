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

impl CreateClassControllerParams {
    pub fn validate(&self) -> Result<&Self, &'static str> {
        if self.code.len() == 0 {
            return Err("code is required");
        }

        if self.name.len() == 0 {
            return Err("name is required");
        }

        if self.description.len() == 0 {
            return Err("description is required");
        }

        Ok(self)
    }
}

pub async fn create_class(
    Extension(current_user): Extension<AuthState>,
    Form(params): Form<CreateClassControllerParams>,
) -> impl IntoResponse {
    let valid_params = match params.validate() {
        Err(err) => return (StatusCode::BAD_REQUEST, err).into_response(),
        Ok(params) => params,
    };

    let class = Class::new(
        &valid_params.code,
        &valid_params.name,
        &valid_params.description,
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
