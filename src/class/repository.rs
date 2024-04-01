use crate::infra::db::get_database;

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

pub async fn get_by_user_id(user_id: &str) -> Result<Vec<Class>, Box<dyn Error>> {
    let class = sqlx::query_as!(
        Class,
        r#"
        SELECT * 
        FROM class 
        WHERE user_id = $1;
        "#,
        user_id
    )
    .fetch_all(get_database())
    .await?;

    Ok(class)
}
