use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Deserializer};

use crate::{user::model::User, utils::env::ENV};

#[derive(Deserialize, Debug)]
struct GithubAuthorizationResponse {
    access_token: String,
}

#[derive(Debug)]
pub struct GithubUserResponse {
    pub id: String,
    pub name: String,
    pub avatar_url: String,
}

impl GithubUserResponse {
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

impl Into<User> for GithubUserResponse {
    fn into(self) -> User {
        User {
            id: self.get_id().to_string(),
            avatar_url: self.get_avatar_url().to_string(),
            name: self.get_name().to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for GithubUserResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            id: u64,
            name: String,
            avatar_url: String,
        }

        let inner = Inner::deserialize(deserializer)?;
        Ok(GithubUserResponse {
            id: inner.id.to_string(),
            name: inner.name,
            avatar_url: inner.avatar_url,
        })
    }
}

pub async fn get_github_access_token(code: &String) -> Result<String, Box<dyn Error>> {
    let url = "https://github.com/login/oauth/access_token";

    let client = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("client_id", ENV.github_client_id());
    body.insert("client_secret", ENV.github_client_secret());
    body.insert("code", code);

    let response = client
        .post(url)
        .json(&body)
        .header("Accept", "application/json")
        .send()
        .await;

    let response = match response {
        Err(_) => return Err("Error when trying to connect with github".into()),
        Ok(response) => response,
    };

    let response: GithubAuthorizationResponse = match response.json().await {
        Err(_) => return Err("Error when trying to get the github response".into()),
        Ok(response) => response,
    };

    Ok(response.access_token)
}

pub async fn get_github_user(token: &str) -> Result<GithubUserResponse, Box<dyn Error>> {
    let url = "https://api.github.com/user";

    let client = reqwest::Client::new();

    let auth_header = format!("Bearer {}", token);
    let response = client
        .get(url)
        .header("Authorization", auth_header)
        .header("user-agent", "reqwest")
        .send()
        .await;

    let response = match response {
        Err(_) => return Err("Error when trying to connect with github".into()),
        Ok(response) => response,
    };

    let response: GithubUserResponse = match response.json().await {
        Err(_) => {
            return Err("Error when trying to retrieve user".into());
        }
        Ok(response) => response,
    };

    Ok(response)
}
