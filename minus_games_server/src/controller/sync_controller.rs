use crate::app_state::AppState;
use crate::auth::auth_layer::AuthLayer;
use crate::auth::user::ArcUser;
use axum::body::Body;
use axum::extract::multipart::Field;
use axum::extract::{DefaultBodyLimit, Multipart, Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use mime::APPLICATION_OCTET_STREAM;
use minus_games_models::sync_file_info::{SyncFileInfo, create_sync_file_infos_from_path};
use minus_games_utils::set_file_modified_time;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;
use tracing::info;
use utoipa::ToSchema;

pub fn new_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/{game}/{folder_hash}", get(get_sync_files_for_folder))
        .route("/{game}/{folder_hash}", post(post_sync_file_for_folder))
        .route("/{game}/{folder_hash}/{*file_path}", get(get_sync_file))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 512))
        .layer(AuthLayer::new(
            app_state.user_handler.clone(),
            app_state.session_manager.clone(),
            app_state.clear_sessions.clone(),
        ))
        .with_state(app_state)
}

#[utoipa::path(
    get,
    path = "/{game}/{folder_hash}/{file_path}",
    params(("game", description = "Game name"), ("folder_hash", description = "Folder Hash"), ("file_path", description = "Path to save file")),
    responses((status = 200, description = "File"), (status = 404, description = "File not Found")),
    context_path = "/sync"
)]
#[axum::debug_handler]
async fn get_sync_file(
    Path((game, folder_hash, file_path)): Path<(String, String, String)>,
    State(app_state): State<Arc<AppState>>,
    user: ArcUser,
) -> Response {
    let save_path = app_state
        .config
        .data_folder
        .join(&user.username)
        .join(game)
        .join(folder_hash)
        .join(file_path);
    info!("Download sync file {}", save_path.display());
    send_file(save_path).await
}

async fn send_file(save_path: PathBuf) -> Response {
    // `File` implements `AsyncRead`
    let filename = match save_path.file_name() {
        Some(filename) => filename,
        None => return (StatusCode::BAD_REQUEST, "Bad Path").into_response(),
    };
    let file = match tokio::fs::File::open(&save_path).await {
        Ok(file) => file,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, APPLICATION_OCTET_STREAM.as_ref()),
        (
            header::CONTENT_DISPOSITION,
            &format!("attachment; filename=\"{filename:?}\""),
        ),
    ];

    (headers, body).into_response()
}

#[utoipa::path(
get,
path = "/{game}/{folder_hash}",
params(("game", description = "Game name"), ("folder_hash", description = "Folder Hash")),
responses((status = 200, description = "File", body = Option < Vec < SyncFileInfo >> ), (status = 404, description = "Folder not Found")),
context_path = "/sync"
)]
#[axum::debug_handler]
async fn get_sync_files_for_folder(
    Path((game, folder_hash)): Path<(String, String)>,
    State(app_state): State<Arc<AppState>>,
    user: ArcUser,
) -> Json<Option<Vec<SyncFileInfo>>> {
    let save_path = app_state
        .config
        .data_folder
        .join(&user.username)
        .join(game)
        .join(folder_hash);
    let save_path_ref = save_path.as_path();
    if save_path_ref.is_dir() {
        let sync_files_infos = create_sync_file_infos_from_path(save_path.as_path());
        Json::from(Some(sync_files_infos))
    } else {
        Json::from(None)
    }
}

#[derive(ToSchema)]
#[allow(dead_code)]
pub struct UploadSyncFile {
    file_name: String,
    file_path: String,
    size: u64,
    last_modified: DateTime<Utc>,
    upload_data: Vec<u8>,
}

#[utoipa::path(
post,
path = "/{game}/{folder_hash}",
params(("game", description = "Game name"), ("folder_hash", description = "Folder Hash")),
request_body(content = UploadSyncFile, content_type = "multipart/form-data"),
responses((status = 200 )),
context_path = "/sync"
)]
#[axum::debug_handler]
async fn post_sync_file_for_folder(
    State(app_state): State<Arc<AppState>>,
    Path((game, folder_hash)): Path<(String, String)>,
    user: ArcUser,
    mut multipart: Multipart,
) -> Response {
    if game.is_empty() || folder_hash.is_empty() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Invalid game or folder hash"))
            .unwrap();
    }

    let mut file_name: Option<String> = None;
    let mut file_path: Option<String> = None;
    let mut size: Option<u64> = None;
    let mut last_modified: Option<SystemTime> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap();
        match field_name {
            "file_name" => {
                file_name = Some(field.text().await.unwrap());
            }
            "file_path" => {
                file_path = Some(field.text().await.unwrap());
            }
            "size" => {
                size = Some(field.text().await.unwrap().parse().unwrap());
            }
            "last_modified" => {
                let last_modified_str = field.text().await.unwrap();
                let last_modified_date_time =
                    DateTime::parse_from_rfc3339(&last_modified_str).unwrap();
                last_modified = Some(last_modified_date_time.into());
            }
            "upload_data" => {
                if file_name.is_none()
                    || file_path.is_none()
                    || size.is_none()
                    || last_modified.is_none()
                {
                    return Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing field"))
                        .unwrap();
                }

                let file_path = app_state
                    .config
                    .data_folder
                    .join(&user.username)
                    .join(&game)
                    .join(&folder_hash)
                    .join(file_path.unwrap());
                write_sync_file(file_path.as_path(), field).await;

                set_file_modified_time(file_path.as_path(), last_modified.unwrap());
                break;
            }
            _ => {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Invalid field name"))
                    .unwrap();
            }
        }
    }

    ().into_response()
}

pub async fn write_sync_file(file_path: &std::path::Path, mut field: Field<'_>) {
    tokio::fs::create_dir_all(file_path.parent().unwrap())
        .await
        .unwrap();
    let mut save_file = File::create(file_path).await.unwrap();
    loop {
        let chunk = field.chunk().await.unwrap();
        match chunk {
            None => {
                break;
            }
            Some(bytes) => {
                let _ = save_file.write(&bytes).await.unwrap();
            }
        }
    }
}
