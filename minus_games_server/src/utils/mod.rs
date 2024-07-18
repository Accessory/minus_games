use crate::auth::user::ArcUser;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

pub async fn super_user_only(user: ArcUser, request: Request, next: Next) -> Response {
    if !user.is_superuser {
        return (StatusCode::FORBIDDEN, "User access forbidden").into_response();
    }
    next.run(request).await
}
