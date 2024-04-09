use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_extra::extract::Form;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{custom::HtmlResponse, exam, middleware::auth::AuthState, view::render_template};

use super::{model::Class, service};

#[derive(Deserialize, Serialize)]
pub struct EnrollClassControllerParams {
    class_id: String,
}

impl EnrollClassControllerParams {
    pub fn validate(&self) -> Result<&Self, &'static str> {
        if self.class_id.len() == 0 {
            return Err("code is required");
        }

        Ok(self)
    }
}

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

pub async fn enroll(
    Extension(current_user): Extension<AuthState>,
    Form(params): Form<EnrollClassControllerParams>,
) -> impl IntoResponse {
    let valid_params = match params.validate() {
        Err(err) => return (StatusCode::BAD_REQUEST, err).into_response(),
        Ok(params) => params,
    };

    match service::enroll_student(&current_user.get_user_id(), &valid_params.class_id).await {
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
        Ok(_) => Redirect::to("/student").into_response(),
    }
}

#[derive(Serialize)]
struct ListExamsHtmlContextModel {
    class_name: String,
    exams: Vec<exam::model::Exam>,
}

pub async fn list_exams(
    Extension(current_user): Extension<AuthState>,
    Path(class_id): Path<String>,
) -> impl IntoResponse {
    match service::is_teacher(&current_user.get_user_id(), &class_id).await {
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
        _ => (),
    };

    let exams = match exam::service::list_exam_by_class_without_relations(&class_id).await {
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(exams) => exams,
    };

    let class = match service::get_by_id(&class_id).await {
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Ok(Some(class)) => class,
    };

    let context = ListExamsHtmlContextModel {
        class_name: class.get_name().to_string(),
        exams,
    };

    render_template("teacher/list-exam", context.into()).to_html_response()
}
