use crate::app_state::AppState;
use crate::auth::auth_layer::AuthLayer;
use crate::auth::user::ArcUser;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;
use utoipa::ToSchema;

pub async fn new_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/upload-saves/{game}", post(post_save_files))
        .route("/upload-save/{game}", post(post_save_file))
        .route("/list", get(get_games_list))
        .nest_service("/data", data_service(app_state.clone()).await)
        .layer(AuthLayer::new(
            app_state.user_handler.clone(),
            app_state.session_manager.clone(),
            app_state.clear_sessions.clone(),
        ))
        .with_state(app_state)
}

#[utoipa::path(
    get,
    path = "/data/{config-file}",
    params(("config-file", description = "Filename")),
    responses((status = 200, description = "File"), (status = 404, description = "File not Found")),
    context_path = "/games"
)]
async fn data_service(app_state: Arc<AppState>) -> ServeDir {
    ServeDir::new(app_state.clone().config.data_folder.as_path())
        .append_index_html_on_directories(false)
}

#[derive(ToSchema)]
#[allow(dead_code)]
pub struct UploadFile {
    saves: Vec<u8>,
}

#[derive(ToSchema)]
#[allow(dead_code)]
pub struct UploadFiles {
    saves: Vec<Vec<u8>>,
}

#[utoipa::path(
    post,
    path = "/upload-saves/{game}",
    request_body(content = UploadFiles, content_type = "multipart/form-data"),
    responses((status = 200, description = "Upload successful")),
    context_path = "/games",
    security(("basic-auth" = []))
)]
#[axum::debug_handler]
pub async fn post_save_files(
    State(app_state): State<Arc<AppState>>,
    game: Path<String>,
    mut multipart: Multipart,
) -> Result<(), StatusCode> {
    if !app_state.does_game_exist(game.as_str()) {
        return Err(StatusCode::NOT_FOUND);
    }
    while let Some(field) = multipart.next_field().await.unwrap() {
        app_state.write_save_file(game.as_str(), field).await;
    }
    Ok(())
}

#[utoipa::path(
    post,
    path = "/upload-save/{game}",
    request_body(content = UploadFile, content_type = "multipart/form-data"),
    responses((status = 200, description = "Upload successful")),
    context_path = "/games",
    security(("basic-auth" = []))
)]
#[axum::debug_handler]
pub async fn post_save_file(
    State(app_state): State<Arc<AppState>>,
    game: Path<String>,
    mut multipart: Multipart,
) -> Result<(), StatusCode> {
    if !app_state.does_game_exist(game.as_str()) {
        return Err(StatusCode::NOT_FOUND);
    }
    if let Some(field) = multipart.next_field().await.unwrap() {
        app_state.write_save_file(game.as_str(), field).await;
    }
    Ok(())
}

#[utoipa::path(
    get,
    path = "/list",
    responses((status = 200, description = "List all existing Games", body = Vec < String >)),
    context_path = "/games",
    security(("basic-auth" = []))
)]
#[axum::debug_handler]
pub async fn get_games_list(
    State(app_state): State<Arc<AppState>>,
    user: ArcUser,
) -> Json<Vec<String>> {
    let path = app_state
        .config
        .data_folder
        .join("*.json")
        .to_str()
        .unwrap()
        .to_string();
    let mut rtn: Vec<String> = Vec::new();
    for entry in glob::glob(&path).unwrap() {
        rtn.push(
            entry
                .unwrap()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }

    Json(user.filter_games_list(rtn))
}
