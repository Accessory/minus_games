use crate::app_state::AppState;
use crate::auth::auth_layer::AuthLayer;
use crate::auth::session_manager::SessionManager;
use crate::auth::user::ArcUser;
use crate::auth::user::User;
use crate::auth::user_handler::UserHandler;
use crate::configuration::Configuration;
use crate::controller::{
    client_controller, download_controller, finder_controller, game_controller, gui_controller,
    sync_controller, updater_controller,
};
use crate::open_api::ApiDoc;
use axum::Router;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use clap::Parser;
use log::{debug, info};
use mime::APPLICATION_JSON;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod app_state;
mod auth;
mod configuration;
mod controller;
mod open_api;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let filter = EnvFilter::default().add_directive(LevelFilter::INFO.into());
    // .add_directive("minus_games_client=debug".parse().unwrap())
    // .add_directive("tower_http::trace=debug".parse().unwrap());
    let config: Configuration = Configuration::parse();

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        // .with_max_level(Level::INFO)
        .init();

    info!("Config:\n{config}");
    let addr = format!("{}:{}", &config.ip, &config.port);
    let user_files_path = config.data_folder.join("users");
    let user_handler = Arc::new(UserHandler { user_files_path });
    let session_manager = Arc::new(RwLock::new(SessionManager::default()));
    let clear_sessions = Arc::new(RwLock::new(None));
    let app_state = Arc::new(AppState {
        config,
        user_handler,
        session_manager,
        clear_sessions,
    });

    // Service
    let service_layers = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
            .on_request(DefaultOnRequest::new().level(Level::TRACE)),
    );

    let app = Router::new()
        .route("/health", get(health))
        .nest("/me", me_route(app_state.clone()).await)
        .nest(
            "/games",
            game_controller::new_router(app_state.clone()).await,
        )
        .nest(
            "/finder",
            finder_controller::new_router(app_state.clone()).await,
        )
        .nest(
            "/download",
            download_controller::new_router(app_state.clone()).await,
        )
        .nest("/sync", sync_controller::new_router(app_state.clone()))
        .nest("/client", client_controller::new_router(app_state.clone()))
        .nest("/gui", gui_controller::new_router(app_state.clone()))
        .nest(
            "/updater",
            updater_controller::new_router(app_state.clone()),
        )
        .route("/", get(redirect_to_openapi))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(service_layers);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn me_route(app_state: Arc<AppState>) -> Router {
    Router::new().route("/", get(me)).layer(AuthLayer::new(
        app_state.user_handler.clone(),
        app_state.session_manager.clone(),
        app_state.clear_sessions.clone(),
    ))
}

async fn redirect_to_openapi() -> Redirect {
    Redirect::permanent("/swagger-ui/")
}

#[utoipa::path(
    get,
    path = "/health",
    responses((status = 200, description = "Server is active and available")),
)]
async fn health() -> Response {
    ().into_response()
}

#[utoipa::path(
    get,
    path = "/me",
    responses((status = 200, description = "Server is active and available", body = User)),
    security(("basic-auth" = []))
)]
async fn me(user: ArcUser) -> Response {
    debug!("{user}");
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(serde_json::to_string(user.deref()).unwrap())
        .unwrap()
        .into_response()
}
