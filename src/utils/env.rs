use once_cell::sync::Lazy;

pub static ENV: Lazy<Env> = Lazy::new(Env::new);

pub struct Env {
    github_client_id: String,
    github_redirect_uri: String,
    github_client_secret: String,
    github_scope: String,
}

impl Env {
    pub fn new() -> Self {
        Self {
            github_client_id: std::env::var("GITHUB_CLIENT_ID")
                .expect("GITHUB_CLIENT_ID must be set."),
            github_redirect_uri: std::env::var("GITHUB_REDIRECT_URI")
                .expect("GITHUB_REDIRECT_URI must be set."),
            github_client_secret: std::env::var("GITHUB_CLIENT_SECRET")
                .expect("GITHUB_CLIENT_SECRET must be set."),
            github_scope: "user:email".into(),
        }
    }

    pub fn github_client_id(&self) -> &String {
        &self.github_client_id
    }

    pub fn github_redirect_uri(&self) -> &String {
        &self.github_redirect_uri
    }

    pub fn github_client_secret(&self) -> &String {
        &self.github_client_secret
    }

    pub fn github_scope(&self) -> &String {
        &self.github_scope
    }
}
