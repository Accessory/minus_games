use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipauto::utoipauto;

// OpenApi
#[utoipauto(paths = "./minus_games_server/src")]
#[derive(OpenApi)]
#[openapi(
    info(description = "Minus Game Server"),
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
