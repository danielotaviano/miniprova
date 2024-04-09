use axum::{extract::Path, response::IntoResponse, Extension, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    class::{self, model::Class},
    custom::HtmlResponse,
    exam::{self, model::Exam},
    middleware::auth::AuthState,
    view::render_template,
};

#[derive(Serialize, Debug)]
struct ClassToSubscribe {
    class: Class,
    count: i64,
}

#[derive(Serialize, Debug)]
struct EnrolledClass {
    class: Class,
    exams: Vec<Exam>,
}

#[derive(Serialize, Debug)]
struct HomeHtmlContextModel {
    classes_to_subscribe: Vec<ClassToSubscribe>,
    enrolled_classes: Vec<EnrolledClass>,
}

#[derive(Serialize, Debug)]
struct ExamHtmlContextModel {
    exam: Exam,
    answers: Vec<String>,
}

#[derive(Serialize, Debug)]
struct ExamResultHtmlContextModel {
    exam: Exam,
    answers: Vec<String>,
    correct_answers_count: i64,
    total_questions: i64,
    correct_answers: Vec<String>,
    score: f64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SaveAnswerBodyParams {
    answer_id: String,
}

pub async fn home_html(Extension(current_user): Extension<AuthState>) -> impl IntoResponse {
    let classes_to_subscribe =
        match class::service::get_by_user_where_is_not_enrolled_with_student_count(
            &current_user.get_user_id(),
        )
        .await
        {
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error getting the classes",
                )
                    .into_response()
            }
            Ok(classes) => classes,
        };

    let enrolled_classes =
        match exam::service::list_exam_group_by_class_without_relations_by_student(
            &current_user.get_user_id(),
        )
        .await
        {
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error getting the student classes",
                )
                    .into_response()
            }
            Ok(classes) => classes,
        };

    let classes_to_subscribe: Vec<_> = classes_to_subscribe
        .into_iter()
        .map(|class| ClassToSubscribe {
            class: class.0,
            count: class.1,
        })
        .collect();

    let enrolled_classes: Vec<_> = enrolled_classes
        .into_iter()
        .map(|class| EnrolledClass {
            class: class.0,
            exams: class.1,
        })
        .collect();

    let context = HomeHtmlContextModel {
        classes_to_subscribe,
        enrolled_classes,
    };

    render_template("student/home", context.into()).to_html_response()
}

pub async fn exam_html(
    Extension(current_user): Extension<AuthState>,
    Path(exam_id): Path<String>,
) -> impl IntoResponse {
    let exam = match exam::service::get_student_exam(&exam_id, &current_user.get_user_id()).await {
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error getting the exam").into_response()
        }
        Ok(exam) => exam,
    };

    let exam = match exam {
        None => return (StatusCode::NOT_FOUND, "Exam not found").into_response(),
        Some(exam) => exam,
    };

    let context = ExamHtmlContextModel {
        exam: exam.0,
        answers: exam.1,
    };

    render_template("student/exam", context.into()).to_html_response()
}

pub async fn exam_result_html(
    Extension(current_user): Extension<AuthState>,
    Path(exam_id): Path<String>,
) -> impl IntoResponse {
    let exam = match exam::service::get_student_exam(&exam_id, &current_user.get_user_id()).await {
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error getting the exam").into_response();
        }
        Ok(exam) => exam,
    };

    let exam = match exam {
        None => return (StatusCode::NOT_FOUND, "Exam not found").into_response(),
        Some(exam) => exam,
    };

    if exam.0.end_date > chrono::Utc::now().timestamp_millis() {
        return (StatusCode::FORBIDDEN, "Exam is not available yet").into_response();
    }

    let correct_answers_count = exam
        .0
        .questions
        .iter()
        .filter(|q| {
            exam.1
                .contains(&q.answers.iter().find(|ans| ans.is_correct).unwrap().id)
        })
        .count() as i64;

    let score = ((correct_answers_count as f64 / exam.0.questions.len() as f64) * 100.0).round();
    let correct_answers: Vec<_> = exam
        .0
        .questions
        .iter()
        .map(|q| q.answers.iter().find(|a| a.is_correct).unwrap().id.clone())
        .collect();

    let context = ExamResultHtmlContextModel {
        exam: exam.0.clone(),
        answers: exam.1,
        correct_answers_count,
        correct_answers,
        total_questions: exam.0.questions.len() as i64,
        score,
    };

    render_template("student/exam_result", context.into()).to_html_response()
}

pub async fn save_answer(
    Extension(current_user): Extension<AuthState>,
    Path((exam_id, question_id)): Path<(String, String)>,
    Json(body): Json<SaveAnswerBodyParams>,
) -> impl IntoResponse {
    match exam::service::save_answer(
        &exam_id,
        &current_user.get_user_id(),
        &question_id,
        &body.answer_id,
    )
    .await
    {
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error saving the answer").into_response();
        }
        Ok(_) => StatusCode::OK.into_response(),
    }
}
