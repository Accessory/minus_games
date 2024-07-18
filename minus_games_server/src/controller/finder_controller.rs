use crate::app_state::AppState;
use crate::auth::auth_layer::AuthLayer;
use crate::utils::super_user_only;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{middleware, Router};
use std::sync::{Arc, LazyLock};
use tracing::info;

pub async fn new_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/rerun-finder-for/:game", post(post_rerun_finder_for))
        .route("/rerun-finder", post(post_rerun_finder))
        .route("/rerun-finder-all", post(post_rerun_finder_all))
        .layer(middleware::from_fn(super_user_only))
        .layer(AuthLayer::new(
            app_state.user_handler.clone(),
            app_state.session_manager.clone(),
            app_state.clear_sessions.clone(),
        ))
        .with_state(app_state)
}

static FINDER_IS_RUNNING: LazyLock<tokio::sync::Mutex<()>> =
    LazyLock::new(|| tokio::sync::Mutex::new(()));

#[utoipa::path(
    post,
    path = "/rerun-finder-for/{game}",
    responses((status = 202, description = "Update the infos for a game")),
    context_path = "/finder",
    security(("basic-auth" = []))
)]
#[axum::debug_handler]
pub async fn post_rerun_finder_for(
    game: Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> StatusCode {
    if let Ok(lock) = FINDER_IS_RUNNING.try_lock() {
        let config = minus_games_finder::configuration::Configuration {
            games_folder: app_state.config.games_folder.clone(),
            data_folder: app_state.config.data_folder.clone(),
            cache_folder: app_state.config.cache_folder.clone(),
            cleanup_data_folder: false,
            keep_existing_configs: false,
            filter: Some(game.to_owned()),
        };

        tokio::task::spawn_blocking(move || {
            info!("Rerun Finder");
            minus_games_finder::run(config);
            info!("Finder finished");
            drop(lock)
        });
    } else {
        info!("Finder was already running");
    }
    StatusCode::ACCEPTED
}

#[utoipa::path(
    post,
    path = "/rerun-finder",
    responses((status = 202, description = "Updates the list of available games")),
    context_path = "/finder",
    security(("basic-auth" = []))
)]
#[axum::debug_handler]
pub async fn post_rerun_finder(State(app_state): State<Arc<AppState>>) -> StatusCode {
    if let Ok(lock) = FINDER_IS_RUNNING.try_lock() {
        let config = minus_games_finder::configuration::Configuration {
            games_folder: app_state.config.games_folder.clone(),
            data_folder: app_state.config.data_folder.clone(),
            cache_folder: app_state.config.cache_folder.clone(),
            cleanup_data_folder: false,
            keep_existing_configs: true,
            filter: None,
        };

        tokio::task::spawn_blocking(move || {
            info!("Rerun Finder");
            minus_games_finder::run(config);
            info!("Finder finished");
            drop(lock)
        });
    } else {
        info!("Finder was already running");
    }

    StatusCode::ACCEPTED
}

#[utoipa::path(
    post,
    path = "/rerun-finder-all",
    responses((status = 202, description = "Updates the complete list of available games")),
    context_path = "/finder",
    security(("basic-auth" = []))
)]
#[axum::debug_handler]
pub async fn post_rerun_finder_all(State(app_state): State<Arc<AppState>>) -> StatusCode {
    if let Ok(lock) = FINDER_IS_RUNNING.try_lock() {
        let config = minus_games_finder::configuration::Configuration {
            games_folder: app_state.config.games_folder.clone(),
            data_folder: app_state.config.data_folder.clone(),
            cache_folder: app_state.config.cache_folder.clone(),
            cleanup_data_folder: false,
            keep_existing_configs: false,
            filter: None,
        };

        tokio::task::spawn_blocking(move || {
            info!("Rerun Finder");
            minus_games_finder::run(config);
            info!("Finder finished updating all games");
            drop(lock)
        });
    } else {
        info!("Finder was already running");
    }
    StatusCode::ACCEPTED
}
