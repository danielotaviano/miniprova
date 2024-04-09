use std::{error::Error, time::SystemTime};

use chrono::DateTime;
use std::time::UNIX_EPOCH;

use crate::{
    class::{self, model::Class},
    user::model::User,
};

use super::{model::Exam, repository};

pub async fn get_with_relations(exam_id: &str) -> Result<Option<Exam>, Box<dyn Error>> {
    repository::get_with_relations(exam_id).await
}

pub async fn save(exam: Exam, teacher_id: &str, class_id: &str) -> Result<(), Box<dyn Error>> {
    if !class::service::is_teacher(teacher_id, class_id).await? {
        return Err("Unauthorized".into());
    }

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

pub async fn list_exam_group_by_class_without_relations_by_student(
    student_id: &str,
) -> Result<Vec<(Class, Vec<Exam>)>, Box<dyn Error>> {
    repository::list_exam_group_by_class_without_relations_by_student(student_id).await
}

pub async fn list_exam_by_class_without_relations(
    class_id: &str,
) -> Result<Vec<Exam>, Box<dyn Error>> {
    repository::list_exam_by_class_id_without_relations(class_id).await
}

pub async fn get_student_exam(
    exam_id: &str,
    student_id: &str,
) -> Result<Option<(Exam, Vec<String>)>, Box<dyn Error>> {
    let exam = repository::get_with_relations(exam_id)
        .await?
        .ok_or_else(|| "Exam not found!")?;

    let is_student = class::service::is_student(student_id, &exam.class_id).await?;
    if !is_student {
        return Err("Unauthorized!".into());
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64;

    if !(exam.start_date <= now) {
        return Err("Exam is not available!".into());
    }

    let answers = repository::get_student_answers(student_id, exam_id).await?;

    Ok(Some((exam, answers)))
}

pub async fn get_exam_results_by_teacher(
    exam_id: &str,
    teacher_id: &str,
) -> Result<(Vec<(User, i64, Option<i64>, Option<i64>, i64)>, i64), Box<dyn Error>> {
    let exam = repository::get_with_relations(exam_id)
        .await?
        .ok_or_else(|| "Exam not found!")?;

    let is_teacher = class::service::is_teacher(teacher_id, &exam.class_id).await?;
    if !is_teacher {
        return Err("Unauthorized!".into());
    }

    let results = repository::get_students_results(exam_id).await?;
    let correct_answers: Vec<_> = exam
        .questions
        .iter()
        .map(|q| {
            q.answers
                .iter()
                .find(|a| a.is_correct())
                .unwrap()
                .id
                .clone()
        })
        .collect();

    let final_results: Vec<_> = results
        .into_iter()
        .map(|(student, answers)| {
            let correct_answers_count = answers
                .iter()
                .filter(|a| correct_answers.contains(&a.answer_id))
                .count() as i64;


            (
                student,
                answers.len() as i64,
                answers.iter().map(|a| a.created_at).min().unwrap_or_default(),
                answers.iter().map(|a| a.updated_at).max().unwrap_or_default(),
                correct_answers_count,
            )
        })
        .collect();

    Ok((final_results, exam.questions.len() as i64))
}

pub async fn save_answer(
    exam_id: &str,
    student_id: &str,
    question_id: &str,
    answer_id: &str,
) -> Result<(), Box<dyn Error>> {
    let exam = repository::get_with_relations(exam_id)
        .await?
        .ok_or_else(|| "Exam not found!")?;

    let is_student = class::service::is_student(student_id, &exam.class_id).await?;
    if !is_student {
        return Err("Unauthorized!".into());
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64;

    if !(exam.start_date <= now && exam.end_date >= now) {
        return Err("Exam is not available!".into());
    }

    let question = exam
        .questions
        .iter()
        .find(|q| q.id == question_id)
        .ok_or_else(|| "Question not found!")?;

    question
        .answers
        .iter()
        .find(|a| a.id == answer_id)
        .ok_or_else(|| "Answer not found!")?;

    repository::save_answer(student_id, question_id, answer_id).await
}

pub async fn delete(exam_id: &str, teacher_id: &str) -> Result<(), Box<dyn Error>> {
    let exam = repository::get_with_relations(exam_id)
        .await?
        .ok_or_else(|| "Exam not found!")?;

    let is_teacher = class::service::is_teacher(teacher_id, &exam.class_id).await?;
    if !is_teacher {
        return Err("Unauthorized!".into());
    }

    if exam.start_date
        <= SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64
    {
        return Err("Can't delete an exam that already started!".into());
    }

    repository::delete_with_relations(exam_id).await
}



pub async fn edit(exam: Exam, teacher_id: &str) -> Result<(), Box<dyn Error>> {

    let saved_exam = match repository::get_with_relations(&exam.id).await {
        Ok(Some(exam)) => exam,
        Ok(None) => return Err("Exam not found!".into()),
        Err(_) => return Err("Internal Server Error".into()),
    };

    if saved_exam.start_date
        <= SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64
    {
        return Err("Can't edit an exam that already started!".into());
    }

    if !class::service::is_teacher(teacher_id, &exam.class_id).await? {
        return Err("Unauthorized".into());
    }

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

    repository::delete_with_relations(&exam.id).await?;
    repository::save(exam).await
}
