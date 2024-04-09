use chrono::{DateTime, NaiveDateTime};
use sqlx::{Postgres, Transaction};

use crate::{class::model::Class, infra::db::get_database, user::model::User};
use std::{error::Error, vec};

use super::model::{Answer, Exam, Question, StudentAnswer};

pub async fn list_exam_group_by_class_without_relations_by_student(
    student_id: &str,
) -> Result<Vec<(Class, Vec<Exam>)>, Box<dyn Error>> {
    let rows = sqlx::query!(
        r#"
        select
            c.*,
            json_agg(e.*) "exams"
        from
            "class" c
        inner join exam e on
            e.class_id = c.id
        inner join class_student cs on
            cs.class_id = c.id
        where
            cs.student_id = $1
        group by
            c.id
        "#,
        student_id
    )
    .fetch_all(get_database())
    .await?;

    let payload: Vec<_> = rows
        .into_iter()
        .map(|row| {
            let class = Class::new_with_id(
                &row.id,
                &row.code,
                &row.name,
                &row.description,
                &row.user_id,
            );

            let exams: Vec<Exam> = row
                .exams
                .unwrap_or_else(|| serde_json::Value::Array(vec![]))
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|exam| {
                    let start_date = exam.get("start_date").unwrap().as_str().unwrap().trim();

                    let start_date =
                        NaiveDateTime::parse_from_str(&start_date, "%Y-%m-%dT%H:%M:%S")
                            .unwrap()
                            .and_utc()
                            .timestamp_millis();

                    let end_date = exam.get("end_date").unwrap().as_str().unwrap().trim();
                    let end_date = NaiveDateTime::parse_from_str(&end_date, "%Y-%m-%dT%H:%M:%S")
                        .unwrap()
                        .and_utc()
                        .timestamp_millis();

                    Exam::new_with_id(
                        exam.get("id").unwrap().as_str().unwrap(),
                        exam.get("name").unwrap().as_str().unwrap(),
                        &start_date,
                        &end_date,
                        exam.get("class_id").unwrap().as_str().unwrap(),
                        vec![],
                    )
                })
                .collect();

            (class, exams)
        })
        .collect();

    Ok(payload)
}

pub async fn get_with_relations(exam_id: &str) -> Result<Option<Exam>, Box<dyn Error>> {
    let exam = sqlx::query!(
        r#"
        with questions as (
            select
                q.*,
                jsonb_agg(a.*) "answers"
            from
                question q
            inner join answer a on
                a.question_id = q.id
            group by
                q.id)
            select
                e.*,
                jsonb_agg(q.*) "questions"
            from
                exam e
            inner join questions q on
                q."exam_id" = e.id
            where e.id = $1
            group by
                e.id
        "#,
        exam_id
    )
    .fetch_optional(get_database())
    .await?;

    let exam = exam.map(|row| {
        let start_date = row.start_date.and_utc().timestamp_millis();
        let end_date = row.end_date.and_utc().timestamp_millis();

        let questions = row
            .questions
            .unwrap_or_else(|| serde_json::Value::Array(vec![]))
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|question| {
                let answers = question
                    .get("answers")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|answer| Answer {
                        answer: answer.get("answer").unwrap().as_str().unwrap().to_string(),
                        id: answer.get("id").unwrap().as_str().unwrap().to_string(),
                        is_correct: answer.get("is_correct").unwrap().as_bool().unwrap(),
                        question_id: answer
                            .get("question_id")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                    })
                    .collect();

                Question::new_with_id(
                    question.get("id").unwrap().as_str().unwrap(),
                    question.get("question").unwrap().as_str().unwrap(),
                    question.get("exam_id").unwrap().as_str().unwrap(),
                    answers,
                )
            })
            .collect();

        Exam::new_with_id(
            &row.id,
            &row.name,
            &start_date,
            &end_date,
            &row.class_id,
            questions,
        )
    });

    Ok(exam)
}

pub async fn get_student_answers(
    student_id: &str,
    exam_id: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let rows = sqlx::query!(
        r#"
        SELECT sa.answer_id
        FROM student_answer sa
        INNER JOIN answer a ON sa.answer_id = a.id
        INNER JOIN question q ON sa.question_id = q.id
        WHERE sa.student_id = $1 AND q.exam_id = $2
        "#,
        student_id,
        exam_id
    )
    .fetch_all(get_database())
    .await?;

    let answers: Vec<String> = rows.into_iter().map(|row| row.answer_id).collect();

    Ok(answers)
}

pub async fn list_exam_by_class_id_without_relations(
    class_id: &str,
) -> Result<Vec<Exam>, Box<dyn Error>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, start_date, end_date
        FROM exam
        WHERE class_id = $1
        "#,
        class_id
    )
    .fetch_all(get_database())
    .await?;

    let mut exams = Vec::new();

    for row in rows {
        let exam = Exam {
            id: row.id,
            name: row.name,
            start_date: row.start_date.and_utc().timestamp_millis(),
            end_date: row.end_date.and_utc().timestamp_millis(),
            class_id: class_id.to_string(),
            questions: vec![],
        };

        exams.push(exam);
    }

    Ok(exams)
}

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

pub async fn save_answer(
    student_id: &str,
    question_id: &str,
    answer_id: &str,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"
            INSERT INTO student_answer (student_id, question_id, answer_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (student_id, question_id) DO UPDATE
            SET answer_id = $3, updated_at = now()
        "#,
        student_id,
        question_id,
        answer_id
    )
    .execute(get_database())
    .await?;

    Ok(())
}

pub async fn get_students_results(
    exam_id: &str,
) -> Result<Vec<(User, Vec<StudentAnswer>)>, Box<dyn Error>> {
    let rows = sqlx::query!(
        r#"
        select
            u.*,
            jsonb_agg(sa.*)
        from
            class_student cs
        inner join exam e on
            e.class_id = cs.class_id
        inner join question q on
            q.exam_id = e.id
        inner join "user" u on
            u.id = cs.student_id
        left join student_answer sa on
            sa.student_id = u.id
            and sa.question_id = q.id
        where
            e.id = $1
        group by
            u.id
        "#,
        exam_id
    )
    .fetch_all(get_database())
    .await?;

    let payload: Vec<_> = rows
        .into_iter()
        .map(|row| {
            let user = User::new_with_id(&row.id, &row.name, &row.avatar_url);

            let student_answers: Vec<StudentAnswer> = row
                .jsonb_agg
                .unwrap_or_else(|| serde_json::Value::Array(vec![]))
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|student_answer| StudentAnswer {
                    student_id: student_answer
                        .get("student_id")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                    question_id: student_answer
                        .get("question_id")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                    answer_id: student_answer
                        .get("answer_id")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                    created_at: student_answer
                        .get("created_at")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .parse::<NaiveDateTime>()
                        .unwrap()
                        .and_utc()
                        .timestamp_millis()
                        .into(),
                    updated_at: student_answer
                        .get("updated_at")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .parse::<NaiveDateTime>()
                        .unwrap()
                        .and_utc()
                        .timestamp_millis()
                        .into(),
                })
                .collect();

            (user, student_answers)
        })
        .collect();

    Ok(payload)
}
