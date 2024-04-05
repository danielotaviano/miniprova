use std::error::Error;

use crate::{class::repository, user};

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

pub async fn get_by_user_id_with_student_count(
    user_id: &str,
) -> Result<Vec<(Class, i64)>, Box<dyn Error>> {
    repository::get_by_user_id_with_student_count(user_id).await
}

pub async fn get_by_user_where_is_not_enrolled_with_student_count(
    user_id: &str,
) -> Result<Vec<(Class, i64)>, Box<dyn Error>> {
    repository::get_by_student_where_is_not_enrolled_with_student_count(user_id).await
}

pub async fn enroll_student(student_id: &str, class_id: &str) -> Result<(), Box<dyn Error>> {
    let user = user::service::get_user(student_id)
        .await?
        .ok_or("Student not found")?;
    let class = repository::get_class(class_id)
        .await?
        .ok_or("Class not found")?;

    repository::enroll_student(&user.get_id(), &class.get_id()).await?;

    Ok(())
}

pub async fn is_teacher(teacher_id: &str, class_id: &str) -> Result<bool, Box<dyn Error>> {
    repository::is_teacher(teacher_id, class_id).await
}
