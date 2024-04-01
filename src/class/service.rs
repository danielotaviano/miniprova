use std::error::Error;

use crate::class::repository;

use super::model::Class;

pub async fn create(class: Class) -> Result<(), Box<dyn Error>> {
    let code_already_in_use = repository::get_by_code(&class.code).await?;

    if let Some(_) = code_already_in_use {
        return Err("Code is already in use!".into());
    }

    match repository::save(class).await {
        Ok(_) => Ok(()),
        Err(_) => Err("Error when trying to create the class!".into()),
    }
}

pub async fn get_by_user_id(user_id: &str) -> Result<Vec<Class>, Box<dyn Error>> {
    repository::get_by_user_id(user_id).await
}
