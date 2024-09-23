use crate::auth;
use crate::controller;
use minus_games_models::sync_file_info;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

// OpenApi
#[derive(OpenApi)]
#[openapi(
    info(description = "Minus Game Server"),
    paths(
        controller::game_controller::post_save_files,
        controller::game_controller::post_save_file,
        controller::game_controller::get_games_list,
        // controller::game_controller::get_file_list,
        controller::game_controller::data_service,
        controller::finder_controller::post_rerun_finder,
        controller::finder_controller::post_rerun_finder_all,
        controller::finder_controller::post_rerun_finder_for,
        controller::finder_controller::post_rerun_finder_for_game,
        controller::download_controller::download_service,
        controller::sync_controller::get_sync_files_for_folder,
        controller::sync_controller::post_sync_file_for_folder,
        controller::sync_controller::get_sync_file,
        controller::updater_controller::get_updater_info_linux,
        controller::updater_controller::get_updater_info_windows,
        controller::client_controller::get_client_info_linux,
        controller::client_controller::get_client_info_windows,
        controller::gui_controller::get_gui_info_linux,
        controller::gui_controller::get_gui_info_windows,
        crate::health,
        crate::me
    ),
    tags(),
    components(schemas(
        sync_file_info::SyncFileInfo,
        controller::sync_controller::UploadSyncFile,
        controller::game_controller::UploadFiles,
        controller::game_controller::UploadFile,
        controller::finder_controller::RerunFinderForGame,
        auth::user::User
    )),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "basic-auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Basic)),
        )
    }
}
