use serde::Serialize;

use crate::utils::generate_id;

#[derive(Serialize, Debug)]
pub struct Class {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub user_id: String,
}

impl Class {
    pub fn new(code: &str, name: &str, description: &str, user_id: &str) -> Self {
        Self {
            id: generate_id(),
            name: name.to_string(),
            description: description.to_string(),
            code: code.to_string(),
            user_id: user_id.to_string(),
        }
    }

    pub fn new_with_id(id: &str, code: &str, name: &str, description: &str, user_id: &str)  -> Self{
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            code: code.to_string(),
            user_id: user_id.to_string(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }
}
