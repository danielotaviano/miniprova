use crate::infra::db::get_database;

use super::model::User;
use std::error::Error;

pub async fn save(user: User) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        r#"
    INSERT INTO "user" (id, name, avatar_url)
    VALUES ($1, $2, $3)
    ON CONFLICT (id) DO UPDATE SET name=excluded.name, avatar_url=excluded.avatar_url
"#,
    )
    .bind(user.get_id())
    .bind(user.get_name())
    .bind(user.get_avatar_url())
    .execute(get_database())
    .await?;

    Ok(())
}

pub async fn get_user(user_id: &str) -> Result<Option<User>, Box<dyn Error>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, avatar_url
        FROM "user"
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(get_database())
    .await?;

    Ok(user)
}
