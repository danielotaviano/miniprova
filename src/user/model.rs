use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub avatar_url: String,
}

impl User {
    pub fn new(id: &str, name: &str, avatar_url: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            avatar_url: avatar_url.to_string(),
        }
    }

    pub fn new_with_id(id: &str, name: &str, avatar_url: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            avatar_url: avatar_url.to_string(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_avatar_url(&self) -> &String {
        &self.avatar_url
    }
}
