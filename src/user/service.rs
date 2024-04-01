use std::error::Error;

use super::{model::User, repository};

pub async fn get_user(user_id: &str) -> Result<Option<User>, Box<dyn Error>> {
    repository::get_user(user_id).await
}

pub async fn save_user(user: User) -> Result<(), Box<dyn Error>> {
    repository::save(user).await
}
