use std::collections::HashMap;

use axum::extract::Request;
use axum_extra::extract::CookieJar;
use std::str::FromStr;
use std::sync::Arc;

use crate::auth::user::User;
use uuid::Uuid;

#[derive(Default)]
pub struct SessionManager {
    pub sessions: HashMap<Uuid, Arc<User>>,
}

impl SessionManager {}

pub static COOKIES_SESSION_NAME: &str = "minus_games_session";

pub fn session_id_from_request(request: &Request) -> Option<Uuid> {
    let cookies = CookieJar::from_headers(request.headers());
    let session_cookie = cookies.get(COOKIES_SESSION_NAME)?;
    Uuid::from_str(session_cookie.value()).ok()
}
