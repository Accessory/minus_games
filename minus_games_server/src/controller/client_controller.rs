use crate::app_state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use minus_games_models::sync_file_info::SyncFileInfo;
use std::sync::Arc;

pub(crate) const TAG: &str = "Client Controller";

pub fn new_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/linux/info", get(get_client_info_linux))
        .route("/windows/info", get(get_client_info_windows))
        .with_state(app_state)
}

#[utoipa::path(
    get,
    path = "/linux/info",
    responses((status = 200, description = "Client Info", body=SyncFileInfo),(status = 404, description = "File not Found")),
    context_path = "/client",
    tag = TAG
)]
#[axum::debug_handler]
async fn get_client_info_linux(State(app_state): State<Arc<AppState>>) -> Response {
    let client_path = app_state.config.games_folder.join("minus_games_client");

    if !client_path.exists() {
        return (
            StatusCode::NOT_FOUND,
            "Minus game updater not found on the server",
        )
            .into_response();
    }

    let cut_off = client_path.iter().count() - 1;

    let sync_file_info = SyncFileInfo::from_path_with_cut_off(client_path, cut_off);

    (StatusCode::OK, Json::from(sync_file_info)).into_response()
}

#[utoipa::path(
    get,
    path = "/windows/info",
    responses((status = 200, description = "Client Info", body=SyncFileInfo),(status = 404, description = "File not Found")),
    context_path = "/client",
    tag = TAG
)]
#[axum::debug_handler]
async fn get_client_info_windows(State(app_state): State<Arc<AppState>>) -> Response {
    let client_path = app_state.config.games_folder.join("minus_games_client.exe");

    if !client_path.exists() {
        return (
            StatusCode::NOT_FOUND,
            "Minus Games Updater not found on the server",
        )
            .into_response();
    }

    let cut_off = client_path.iter().count() - 1;

    let sync_file_info = SyncFileInfo::from_path_with_cut_off(client_path, cut_off);

    (StatusCode::OK, Json::from(sync_file_info)).into_response()
}
