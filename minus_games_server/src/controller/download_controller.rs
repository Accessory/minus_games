use crate::app_state::AppState;
use crate::auth::auth_layer::AuthLayer;
use crate::auth::user::ArcUser;
use axum::Router;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use minus_games_utils::constants::ADDITIONS;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub(crate) const TAG: &str = "Download Controller";

pub async fn new_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest_service("/additions", additions_service(app_state.clone()).await)
        .fallback_service(download_service(app_state.clone()).await)
        .layer(axum::middleware::from_fn(check_download_access))
        .layer(AuthLayer::new(
            app_state.user_handler.clone(),
            app_state.session_manager.clone(),
            app_state.clear_sessions.clone(),
        ))
}

#[utoipa::path(
    get,
    path = "/additions/{game}/{file}",
    params(("game", description = "Game"), ("file", description = "Additional Game File")),
    responses((status = 200, description = "File"), (status = 404, description = "File not Found")),
    context_path = "/download",
    tag = TAG
)]
async fn additions_service(app_state: Arc<AppState>) -> ServeDir {
    ServeDir::new(app_state.clone().config.data_folder.join(ADDITIONS))
        .append_index_html_on_directories(false)
}

async fn check_download_access(user: ArcUser, request: Request, next: Next) -> Response {
    if !user.is_superuser {
        let game_name_encoded = request
            .uri()
            .path()
            .split("/")
            .skip(1)
            .find(|&i| i != ADDITIONS)
            .unwrap_or_default();

        let game_name_decoded = url::form_urlencoded::parse(game_name_encoded.as_bytes())
            .next()
            .unwrap()
            .0
            .to_string();
        if !user.is_game_allowed(&game_name_decoded) {
            // info!("Game {}", game_name_decoded);
            return (
                StatusCode::FORBIDDEN,
                format!("Downloading '{game_name_decoded}' is not allowed"),
            )
                .into_response();
        }
    }

    next.run(request).await
}

#[utoipa::path(
    get,
    path = "/{file}",
    params(("file", description = "Filename")),
    responses((status = 200, description = "File"),(status = 404, description = "File not Found")),
    context_path = "/download",
    tag = TAG
)]
async fn download_service(app_state: Arc<AppState>) -> ServeDir {
    ServeDir::new(app_state.config.games_folder.as_path()).append_index_html_on_directories(false)
}
