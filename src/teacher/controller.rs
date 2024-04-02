use axum::{response::IntoResponse, Extension};
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    class::{self, model::Class},
    custom::HtmlResponse,
    middleware::auth::AuthState,
    view::render_template,
};

#[derive(Serialize)]
struct HomeHtmlContextModel {
    class: Class,
    count: i64,
}

impl From<(Class, i64)> for HomeHtmlContextModel {
    fn from((class, count): (Class, i64)) -> Self {
        Self { class, count }
    }
}

pub async fn home_html(Extension(current_user): Extension<AuthState>) -> impl IntoResponse {
    match class::service::get_by_user_id_with_student_count(&current_user.get_user_id()).await {
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting the classes",
        )
            .into_response(),
        Ok(classes) => {
            let context: Vec<HomeHtmlContextModel> = classes
                .into_iter()
                .map(HomeHtmlContextModel::from)
                .collect();
            render_template("teacher/home", context.into()).to_html_response()
        }
    }
}
