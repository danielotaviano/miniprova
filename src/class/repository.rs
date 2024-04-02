use crate::{infra::db::get_database, utils::generate_id};

use std::error::Error;

use super::model::Class;

pub async fn save(class: Class) -> Result<(), Box<dyn Error>> {
    sqlx::query(
    r#"
    INSERT INTO "class" (id, code, name, description, user_id)
    VALUES ($1, $2, $3, $4, $5)
    ON CONFLICT (id) DO UPDATE SET code=excluded.code, name=excluded.name, description=excluded.description
    "#,
    )
    .bind(class.get_id())
    .bind(class.get_code())
    .bind(class.get_name())
    .bind(class.get_description())
    .bind(class.get_user_id())
    .execute(get_database())
    .await?;

    Ok(())
}

pub async fn get_by_code(code: &str) -> Result<Option<Class>, Box<dyn Error>> {
    let class = sqlx::query_as!(
        Class,
        r#"
        SELECT * 
        FROM class 
        WHERE code = $1;
        "#,
        code
    )
    .fetch_optional(get_database())
    .await?;

    Ok(class)
}

pub async fn get_by_user_id_with_student_count(
    user_id: &str,
) -> Result<Vec<(Class, i64)>, Box<dyn Error>> {
    let rows = sqlx::query!(
        r#"
        select
            c.*, count(cs.id) "student_total"
        from
            "class" c
        left join class_student cs on
            cs.class_id = c.id
        where
            user_id = $1
            group by c.id;
        "#,
        user_id
    )
    .fetch_all(get_database())
    .await?;

    let classes_with_student_count: Vec<_> = rows
        .into_iter()
        .map(|row| {
            let class = Class::new(&row.code, &row.name, &row.description, &row.user_id);
            let count = row.student_total.unwrap_or(0);

            (class, count)
        })
        .collect();

    Ok(classes_with_student_count)
}

pub async fn get_by_student_where_is_not_enrolled_with_student_count(
    user_id: &str,
) -> Result<Vec<(Class, i64)>, Box<dyn Error>> {
    let rows = sqlx::query!(
        r#"
        select
            c.*, count(cs2.id) "student_total"
        from
            "class" c
        left join class_student cs on
            cs.class_id = c.id
            and cs.student_id = $1
        left join class_student cs2 on
            cs2.class_id = c.id
        where
            cs.student_id is null
            group by c.id;

        "#,
        user_id
    )
    .fetch_all(get_database())
    .await?;

    let classes_with_student_count: Vec<_> = rows
        .into_iter()
        .map(|row| {
            let class = Class::new(&row.code, &row.name, &row.description, &row.user_id);
            let count = row.student_total.unwrap_or(0);

            (class, count)
        })
        .collect();

    Ok(classes_with_student_count)
}

pub async fn enroll_student(student_id: &str, class_id: &str) -> Result<(), Box<dyn Error>> {
    let id = generate_id();
    sqlx::query!(
        r#"
        INSERT INTO class_student (id, student_id, class_id)
        VALUES ($1, $2, $3)
        "#,
        id,
        student_id,
        class_id
    )
    .execute(get_database())
    .await?;

    Ok(())
}

pub async fn get_class(class_id: &str) -> Result<Option<Class>, Box<dyn Error>> {
    let class = sqlx::query_as!(
        Class,
        r#"
        SELECT * 
        FROM class 
        WHERE id = $1;
        "#,
        class_id
    )
    .fetch_optional(get_database())
    .await?;

    Ok(class)
}
