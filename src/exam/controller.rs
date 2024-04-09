use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Json, Path},
    response::{IntoResponse, Redirect},
    Extension,
};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    class,
    custom::HtmlResponse,
    exam::model::{Answer, Question},
    middleware::auth::AuthState,
    view::render_template,
};

use super::{model::Exam, repository, service};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateAnswerControllerParams {
    answer: String,
    #[serde(rename = "isCorrect")]
    is_correct: bool,
}

impl CreateAnswerControllerParams {
    pub fn validate(&self) -> Result<&Self, &'static str> {
        if self.answer.is_empty() {
            return Err("Answer cannot be empty");
        }

        if self.is_correct && self.answer.is_empty() {
            return Err("Correct answer cannot be empty");
        }

        Ok(self)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateQuestionControllerParams {
    question: String,
    answers: Vec<CreateAnswerControllerParams>,
}

impl CreateQuestionControllerParams {
    pub fn validate(&self) -> Result<&Self, &'static str> {
        if self.question.is_empty() {
            return Err("Question cannot be empty");
        }

        for answer in &self.answers {
            if let Err(err) = answer.validate() {
                return Err(err);
            }
        }

        Ok(self)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateExamControllerParams {
    name: String,
    #[serde(rename = "startDate")]
    start_date: i64,
    #[serde(rename = "endDate")]
    end_date: i64,
    questions: Vec<CreateQuestionControllerParams>,
}

impl CreateExamControllerParams {
    pub fn validate(&self) -> Result<&Self, &'static str> {
        if self.name.is_empty() {
            return Err("Exam name cannot be empty");
        }

        for question in &self.questions {
            if let Err(err) = question.validate() {
                return Err(err);
            }
        }

        Ok(self)
    }
}

pub async fn edit(
    Extension(current_user): Extension<AuthState>,
    Path(exam_id): Path<String>,
    Json(params): Json<CreateExamControllerParams>,
) -> impl IntoResponse {
    let valid_params = match params.validate() {
        Err(err) => return (StatusCode::BAD_REQUEST, err).into_response(),
        Ok(params) => params,
    };

    let saved_exam = match service::get_with_relations(&exam_id).await {
        Ok(Some(exam)) => exam,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let mut exam = Exam::new_with_id(
        &exam_id,
        &valid_params.name,
        &valid_params.start_date,
        &valid_params.end_date,
        &saved_exam.class_id,
        vec![],
    );

    let questions: Vec<_> = valid_params
        .questions
        .iter()
        .map(|question| {
            let mut question_model = Question::new(&question.question, exam.get_id(), vec![]);

            let answers: Vec<_> = question
                .answers
                .iter()
                .map(|answer| {
                    Answer::new(&answer.answer, &answer.is_correct, question_model.get_id())
                })
                .collect();

            question_model.set_answers(answers);

            question_model
        })
        .collect();

    exam.set_questions(questions);

    match service::edit(exam, &current_user.get_user_id()).await {
        Err(_) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Ok(_) => StatusCode::OK.into_response(),
    }
}

pub async fn create(
    Extension(current_user): Extension<AuthState>,
    Path(class_id): Path<String>,
    Json(params): Json<CreateExamControllerParams>,
) -> impl IntoResponse {
    let valid_params = match params.validate() {
        Err(err) => return (StatusCode::BAD_REQUEST, err).into_response(),
        Ok(params) => params,
    };

    let mut exam = Exam::new(
        &valid_params.name,
        &valid_params.start_date,
        &valid_params.end_date,
        &class_id,
        vec![],
    );

    let questions: Vec<_> = valid_params
        .questions
        .iter()
        .map(|question| {
            let mut question_model = Question::new(&question.question, exam.get_id(), vec![]);

            let answers: Vec<_> = question
                .answers
                .iter()
                .map(|answer| {
                    Answer::new(&answer.answer, &answer.is_correct, question_model.get_id())
                })
                .collect();

            question_model.set_answers(answers);

            question_model
        })
        .collect();

    exam.set_questions(questions);

    match service::save(exam, &current_user.get_user_id(), &class_id).await {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(_) => StatusCode::OK.into_response(),
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateHtmlControllerPathParams {
    class_id: String,
}

pub async fn create_html(
    Extension(current_user): Extension<AuthState>,
    Path(params): Path<CreateHtmlControllerPathParams>,
) -> impl IntoResponse {
    let is_teacher =
        match class::service::is_teacher(&current_user.get_user_id(), &params.class_id).await {
            Ok(is_teacher) => is_teacher,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

    if !is_teacher {
        return Redirect::to("/teacher").into_response();
    }

    let class = match class::service::get_by_id(&params.class_id).await {
        Ok(Some(class)) => class,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    render_template("exam/create", class.into()).to_html_response()
}

pub async fn delete(
    Extension(current_user): Extension<AuthState>,
    Path(exam_id): Path<String>,
) -> impl IntoResponse {
    match service::delete(&exam_id, &current_user.get_user_id()).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn edit_html(
    Extension(current_user): Extension<AuthState>,
    Path(exam_id): Path<String>,
) -> impl IntoResponse {
    let exam = match repository::get_with_relations(&exam_id).await {
        Ok(Some(exam)) => exam,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let is_teacher = class::service::is_teacher(&current_user.get_user_id(), &exam.class_id).await;

    if !is_teacher.unwrap_or(false) {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    if exam.start_date
        <= SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64
    {
        return StatusCode::BAD_REQUEST.into_response();
    }

    render_template("teacher/edit-exam", exam.into()).to_html_response()
}
