use chrono::{DateTime, NaiveDateTime};
use sqlx::{Postgres, Transaction};

use crate::infra::db::get_database;
use std::error::Error;

use super::model::{Answer, Exam, Question};

pub async fn save(exam: Exam) -> Result<(), Box<dyn Error>> {
    let mut tx = get_database().begin().await?;

    let start_date = DateTime::from_timestamp_millis(*exam.get_start_date())
        .ok_or_else(|| format!("Invalid start date: {}", *exam.get_start_date()))?
        .naive_utc();
    let end_date = DateTime::from_timestamp_millis(*exam.get_end_date())
        .ok_or_else(|| format!("Invalid end date: {}", *exam.get_end_date()))?
        .naive_utc();

    insert_exam(&mut tx, &exam, start_date, end_date).await?;
    insert_questions(&mut tx, exam.get_questions()).await?;
    insert_answers(
        &mut tx,
        exam.get_questions()
            .into_iter()
            .flat_map(|q| q.get_answers())
            .collect(),
    )
    .await?;

    tx.commit().await?;

    Ok(())
}

async fn insert_exam(
    tx: &mut Transaction<'static, Postgres>,
    exam: &Exam,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"
            INSERT INTO exam (id, name, start_date, end_date, class_id)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        exam.get_id(),
        exam.get_name(),
        start_date,
        end_date,
        exam.get_class_id()
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_questions(
    tx: &mut Transaction<'static, Postgres>,
    questions: &Vec<Question>,
) -> Result<(), Box<dyn Error>> {
    let question_tuple =
        questions
            .into_iter()
            .fold((vec![], vec![], vec![]), |mut acc, question| {
                acc.0.push(question.get_id().clone());
                acc.1.push(question.get_question().clone());
                acc.2.push(question.get_exam_id().clone());

                acc
            });

    sqlx::query!(
        r#"
            INSERT INTO question (id, question, exam_id)
            SELECT * FROM UNNEST($1::varchar[], $2::varchar[], $3::varchar[])
        "#,
        &question_tuple.0,
        &question_tuple.1,
        &question_tuple.2
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_answers(
    tx: &mut Transaction<'static, Postgres>,
    answers: Vec<&Answer>,
) -> Result<(), Box<dyn Error>> {
    let answer_tuple =
        answers
            .into_iter()
            .fold((vec![], vec![], vec![], vec![]), |mut acc, answer| {
                acc.0.push(answer.get_id().clone());
                acc.1.push(answer.get_answer().clone());
                acc.2.push(answer.is_correct().clone());
                acc.3.push(answer.get_question_id().clone());

                acc
            });

    sqlx::query!(
        r#"
            INSERT INTO answer (id, answer, is_correct, question_id)
            SELECT * FROM UNNEST($1::varchar[], $2::varchar[], $3::bool[], $4::varchar[])
        "#,
        &answer_tuple.0,
        &answer_tuple.1,
        &answer_tuple.2,
        &answer_tuple.3
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
