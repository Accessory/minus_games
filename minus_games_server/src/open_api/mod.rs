use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipauto::utoipauto;

// OpenApi
#[utoipauto(paths = "./minus_games_server/src")]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = crate::TAG, description = "Main Controller Routes"),
        (name = crate::controller::game_controller::TAG, description = "Game Controller Routes"),
        (name = crate::controller::gui_controller::TAG, description = "Gui Controller Routes"),
        (name = crate::controller::client_controller::TAG, description = "Client Controller Routes"),
        (name = crate::controller::download_controller::TAG, description = "Downloader Controller Routes"),
        (name = crate::controller::finder_controller::TAG, description = "Finder Controller Routes"),
        (name = crate::controller::sync_controller::TAG, description = "Sync Controller Routes"),
        (name = crate::controller::updater_controller::TAG, description = "Updater Controller Routes")
    ),
    info(title = "Minus Games Server", description = "Minus Game Server"),
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
