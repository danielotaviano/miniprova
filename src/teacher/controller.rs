use axum::{extract::Path, response::IntoResponse, Extension};
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    class::{self, model::Class},
    custom::HtmlResponse,
    exam::{self, model::StudentAnswer},
    middleware::auth::AuthState,
    user::model::User,
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

#[derive(Serialize)]
pub struct StudentResultContextModel {
    pub name: String,
    pub answers_count: i64,
    pub first_answer_date: Option<i64>,
    pub last_update: Option<i64>,
    pub correct_answers_count: i64,
}

#[derive(Serialize)]

pub struct ExamResultHtmlContextModel {
    pub students: Vec<StudentResultContextModel>,
    pub total_questions: i64,
}

impl From<(Vec<(User, i64, Option<i64>, Option<i64>, i64)>, i64)> for ExamResultHtmlContextModel {
    fn from(
        (students, total_questions): (Vec<(User, i64, Option<i64>, Option<i64>, i64)>, i64),
    ) -> Self {
        let students = students
            .into_iter()
            .map(
                |(
                    student,
                    answers_count,
                    first_answer_date,
                    last_update,
                    correct_answers_count,
                )| {
                    StudentResultContextModel {
                        name: student.name,
                        answers_count,
                        first_answer_date,
                        last_update,
                        correct_answers_count,
                    }
                },
            )
            .collect();
        Self {
            students,
            total_questions,
        }
    }
}

pub async fn exam_results_html(
    Extension(current_user): Extension<AuthState>,
    Path(exam_id): Path<String>,
) -> impl IntoResponse {
    match exam::service::get_exam_results_by_teacher(&exam_id, &current_user.get_user_id()).await {
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting the exam results",
        )
            .into_response(),
        Ok(results) => {
            let context: ExamResultHtmlContextModel = results.into();
            render_template("teacher/exam_result", context.into()).to_html_response()
        }
    }
}
