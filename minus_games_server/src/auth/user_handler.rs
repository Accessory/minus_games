use crate::auth::user::User;
use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use minus_games_utils::verify_argon2_hash;
use std::io::BufReader;
use std::path::PathBuf;
use tracing::trace;

pub struct UserHandler {
    pub user_files_path: PathBuf,
}

pub const DEFAULT_USERNAME: &str = "default";
pub const DEFAULT_PASSWORD: &str = "default";

impl UserHandler {
    pub fn get_authorization_from_request(&self, request: &Request) -> Option<(String, String)> {
        let authorization = request.headers().get(AUTHORIZATION)?;
        let auth = authorization.to_str().ok()?;
        let mut split = auth.split(' ');
        let auth_type = split.next()?;

        if auth_type != "Basic" {
            return None;
        }

        let auth_value_base64 = split.next()?;
        let auth_value = String::from_utf8(BASE64_STANDARD.decode(auth_value_base64).ok()?).ok()?;

        let mut auth_value = auth_value.split(':');
        let username = auth_value.next()?.to_string();
        let password = auth_value.next()?.to_string();

        Some((username, password))
    }

    pub fn get_default_user(&self) -> User {
        self.authorize_user_by_username_password(DEFAULT_USERNAME, DEFAULT_PASSWORD)
            .unwrap_or_default()
    }

    pub fn authorize_user_by_username_password(
        &self,
        username: &str,
        password: &str,
    ) -> Option<User> {
        trace!("Authorized: {username} - {password}");
        let user_file_path = self.user_files_path.join(format!("{username}.json"));
        let user_file = std::fs::File::open(user_file_path).ok()?;
        let reader = BufReader::new(user_file);

        let user: User = serde_json::from_reader(reader).ok()?;
        if verify_argon2_hash(password, user.password.as_str()) {
            return Some(user);
        }
        None
    }
}
