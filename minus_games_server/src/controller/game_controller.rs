use crate::app_state::AppState;
use crate::auth::auth_layer::AuthLayer;
use crate::auth::user::ArcUser;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use log::warn;
use minus_games_models::game_list::{
    GamesWithDate, GamesWithGameInfos, GamesWithInfos, GamesWithMinimalGameInfos,
};
use std::sync::Arc;
use tower_http::services::ServeDir;
use utoipa::ToSchema;

pub(crate) const TAG: &str = "Game Controller";

pub async fn new_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/upload-saves/{game}", post(post_save_files))
        .route("/upload-save/{game}", post(post_save_file))
        .route("/list", get(get_games_list))
        .route("/list-with-date", get(get_ordered_games_list))
        .route("/list-with-infos", get(get_ordered_games_infos_list))
        .route(
            "/list-with-game-infos",
            get(get_ordered_with_games_infos_list),
        )
        .route(
            "/list-with-minimal-game-infos",
            get(get_ordered_with_minimal_games_infos_list),
        )
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
    context_path = "/games",
    tag = TAG
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
    security(("basic-auth" = [])),
    tag = TAG
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
    security(("basic-auth" = [])),
    tag = TAG
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
    security(("basic-auth" = [])),
    tag = TAG
)]
#[axum::debug_handler]
pub async fn get_games_list(
    State(app_state): State<Arc<AppState>>,
    user: ArcUser,
) -> Json<Vec<String>> {
    let game_list = app_state.config.get_game_list();
    Json(user.filter_games_list(game_list))
}

#[utoipa::path(
    get,
    path = "/list-with-date",
    responses((status = 200, description = "List all existing Games", body = Vec < GamesWithDate >)),
    context_path = "/games",
    security(("basic-auth" = [])),
    tag = TAG
)]
#[axum::debug_handler]
pub async fn get_ordered_games_list(
    State(app_state): State<Arc<AppState>>,
    user: ArcUser,
) -> Json<Vec<GamesWithDate>> {
    let game_list = app_state.config.get_game_list();
    let filtered_game_list = user.filter_games_list(game_list);
    let mut rtn = Vec::with_capacity(filtered_game_list.len());
    for name in filtered_game_list {
        let modification_date = app_state.config.get_modification_date_for_game(&name);
        rtn.push(GamesWithDate::new(name, modification_date));
    }
    Json(rtn)
}

#[utoipa::path(
    get,
    path = "/list-with-infos",
    responses((status = 200, description = "List all existing Games with infos", body = Vec < GamesWithInfos >)),
    context_path = "/games",
    security(("basic-auth" = [])),
    tag = TAG
)]
#[axum::debug_handler]
pub async fn get_ordered_games_infos_list(
    State(app_state): State<Arc<AppState>>,
    user: ArcUser,
) -> Json<Vec<GamesWithInfos>> {
    let game_list = app_state.config.get_game_list();
    let filtered_game_list = user.filter_games_list(game_list);
    let mut rtn = Vec::with_capacity(filtered_game_list.len());
    for name in filtered_game_list {
        let modification_date = app_state.config.get_modification_date_for_game(&name);
        let header_exists = app_state.config.does_game_has_header_image(&name);
        rtn.push(GamesWithInfos::new(name, modification_date, header_exists));
    }
    Json(rtn)
}

#[utoipa::path(
    get,
    path = "/list-with-game-infos",
    responses((status = 200, description = "List all existing Games with infos", body = Vec < GamesWithGameInfos >)),
    context_path = "/games",
    security(("basic-auth" = [])),
    tag = TAG
)]
#[axum::debug_handler]
pub async fn get_ordered_with_games_infos_list(
    State(app_state): State<Arc<AppState>>,
    user: ArcUser,
) -> Json<Vec<GamesWithGameInfos>> {
    let game_list = app_state.config.get_game_list();
    let filtered_game_list = user.filter_games_list(game_list);
    let mut rtn = Vec::with_capacity(filtered_game_list.len());
    for name in filtered_game_list {
        let modification_date = app_state.config.get_modification_date_for_game(&name);
        let header_exists = app_state.config.does_game_has_header_image(&name);
        if let Some(game_infos) = app_state.get_game_infos(&name) {
            rtn.push(GamesWithGameInfos::new(
                name,
                modification_date,
                header_exists,
                game_infos,
            ));
        } else {
            warn!("There are no game infos for {name}");
        }
    }
    Json(rtn)
}

#[utoipa::path(
    get,
    path = "/list-with-minimal-game-infos",
    responses((status = 200, description = "List all existing Games with infos", body = Vec < GamesWithMinimalGameInfos >)),
    context_path = "/games",
    security(("basic-auth" = [])),
    tag = TAG
)]
#[axum::debug_handler]
pub async fn get_ordered_with_minimal_games_infos_list(
    State(app_state): State<Arc<AppState>>,
    user: ArcUser,
) -> Json<Vec<GamesWithMinimalGameInfos>> {
    let game_list = app_state.config.get_game_list();
    let filtered_game_list = user.filter_games_list(game_list);
    let mut rtn = Vec::with_capacity(filtered_game_list.len());
    for name in filtered_game_list {
        let modification_date = app_state.config.get_modification_date_for_game(&name);
        let header_exists = app_state.config.does_game_has_header_image(&name);
        if let Some(game_infos) = app_state.get_game_infos(&name) {
            rtn.push(GamesWithMinimalGameInfos::new(
                name,
                modification_date,
                header_exists,
                game_infos.into(),
            ));
        } else {
            warn!("There are no game infos for {name}");
        }
    }
    Json(rtn)
}
