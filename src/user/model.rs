use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub avatar_url: String,
}

impl User {
    pub fn new(id: String, name: String, avatar_url: String) -> Self {
        Self {
            id,
            name,
            avatar_url,
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
