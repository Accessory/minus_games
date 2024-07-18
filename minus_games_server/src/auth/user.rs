use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub username: String,
    pub password: String,
    pub include_list: Vec<String>,
    pub exclude_list: Vec<String>,
    pub is_superuser: bool,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "default".to_string(),
            password: "default".to_string(),
            include_list: vec![],
            exclude_list: vec![],
            is_superuser: true,
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Username: {}, Password, {}, include_list {:?}, exclude_list {:?}",
            self.username, self.password, self.include_list, self.exclude_list
        )
    }
}
pub struct ArcUser(Arc<User>);

impl ArcUser {
    pub fn filter_games_list(&self, mut games: Vec<String>) -> Vec<String> {
        if self.is_superuser {
            return games;
        }

        if !self.include_list.is_empty() {
            games.retain_mut(|i| self.include_list.contains(i));
        }

        if !self.exclude_list.is_empty() {
            games.retain_mut(|i| !self.exclude_list.contains(i));
        }

        games
    }

    pub fn is_game_allowed(&self, game: &String) -> bool {
        if self.is_superuser {
            return true;
        }

        if !self.include_list.is_empty() && !self.include_list.contains(game) {
            return false;
        }

        if !self.exclude_list.is_empty() && self.include_list.contains(game) {
            return false;
        }

        true
    }
}

impl Deref for ArcUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Display for ArcUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ArcUser
where
    S: Sync + Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.extensions.get::<Arc<User>>() {
            None => Err((StatusCode::FORBIDDEN, "No user found")),
            Some(user) => Ok(ArcUser(user.clone())),
        }
    }
}
