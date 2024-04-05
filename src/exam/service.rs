use std::error::Error;

use chrono::DateTime;

use crate::class;

use super::{model::Exam, repository};

pub async fn save(exam: Exam) -> Result<(), Box<dyn Error>> {
    let questions = exam.get_questions();

    if questions.is_empty() {
        return Err("Can not create an Exam without questions!".into());
    }

    for question in questions {
        let answers = question.get_answers();
        if answers.is_empty() {
            return Err("Can not create a Exam with a question without answer!".into());
        }

        if !answers.iter().any(|a| a.is_correct()) {
            return Err("Can not create a Exam with a question without a correct answer!".into());
        }
    }

    let start_date = DateTime::from_timestamp(*exam.get_start_date(), 0)
        .ok_or_else(|| format!("Invalid start date: {}", *exam.get_start_date()))?
        .naive_utc();
    let end_date = DateTime::from_timestamp(*exam.get_end_date(), 0)
        .ok_or_else(|| format!("Invalid end date: {}", *exam.get_end_date()))?
        .naive_utc();

    let now = chrono::Utc::now().naive_utc();
    if start_date < now {
        return Err("Start date can't be in the past!".into());
    }

    if end_date < start_date {
        return Err("End date can't be before start date".into());
    }

    let class = class::repository::get_class(exam.get_class_id()).await?;

    if class.is_none() {
        return Err("Class can't be found!".into());
    }

    repository::save(exam).await
}
