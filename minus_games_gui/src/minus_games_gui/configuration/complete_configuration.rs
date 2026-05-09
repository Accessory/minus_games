use crate::minus_games_gui::configuration::gui_configuration::GuiConfiguration;
use crate::minus_games_gui::configuration::{DEFAULT_FONT_NAME, Mode};
use clap::Parser;
use minus_games_client::configuration::{ClientActions, ClientConfiguration};
use minus_games_utils::ClientFolder;
use minus_games_utils::ClientGamesFolder;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CompleteConfiguration {
    #[arg(
        long,
        default_value = "http://127.0.0.1:8415",
        env = "MINUS_GAMES_SERVER_URL"
    )]
    pub server_url: String,
    #[arg(long, default_value = ClientFolder {}, env = "MINUS_GAMES_CLIENT_FOLDER")]
    pub client_folder: PathBuf,
    #[arg(long, env = "MINUS_GAMES_WINE_EXE")]
    pub wine_exe: Option<PathBuf>,
    #[arg(long, env = "MINUS_GAMES_WINE_PREFIX")]
    pub wine_prefix: Option<PathBuf>,
    #[arg(short, long, default_value = "false", env = "MINUS_GAMES_VERBOSE")]
    pub verbose: bool,
    #[arg(short, long, default_value = "false", env = "MINUS_GAMES_OFFLINE")]
    pub offline: bool,
    #[arg(long, default_value = ClientGamesFolder {}, env = "MINUS_GAMES_CLIENT_GAMES_FOLDER")]
    pub client_games_folder: PathBuf,
    #[arg(long, default_value = ClientGamesFolder {}, env = "MINUS_GAMES_CLIENT_CACHE_FOLDER")]
    pub client_cache_folder: Option<PathBuf>,
    #[arg(long, env = "MINUS_GAMES_USERNAME")]
    pub username: Option<String>,
    #[arg(long, env = "MINUS_GAMES_PASSWORD")]
    pub password: Option<String>,
    #[arg(long, default_value = "false", env = "MINUS_GAMES_NO_GAMEMODERUN")]
    pub no_gamemoderun: bool,
    #[arg(long, default_value = "true", env = "MINUS_GAMES_SYNC")]
    pub sync: bool,
    #[command(subcommand)]
    pub action: Option<ClientActions>,
    #[arg(long, env = "MINUS_GAMES_GUI_FULLSCREEN")]
    pub fullscreen: bool,
    #[arg(long, env = "MINUS_GAMES_GUI_MODE", default_value = "Gui")]
    pub mode: Mode,
    #[arg(long, env = "MINUS_GAMES_GUI_THEME")]
    pub theme: Option<String>,
    #[arg(long, env = "MINUS_GAMES_GUI_SCALE")]
    pub scale: Option<f32>,
    #[arg(
        long,
        env = "MINUS_GAMES_GUI_FONT",
        default_value = DEFAULT_FONT_NAME
    )]
    pub font: String,
}

impl CompleteConfiguration {
    pub fn into_gui_configuration_and_client_configuration(
        self,
    ) -> (GuiConfiguration, ClientConfiguration) {
        let gui_configuration = GuiConfiguration {
            fullscreen: self.fullscreen,
            mode: self.mode,
            theme: self.theme,
            scale: self.scale,
            font: self.font,
        };

        let client_configuration = ClientConfiguration {
            server_url: self.server_url,
            client_folder: self.client_folder,
            wine_exe: self.wine_exe,
            wine_prefix: self.wine_prefix,
            verbose: self.verbose,
            offline: self.offline,
            client_games_folder: self.client_games_folder,
            client_cache_folder: self.client_cache_folder,
            username: self.username,
            password: self.password,
            no_gamemoderun: self.no_gamemoderun,
            sync: self.sync,
            action: self.action,
        };

        (gui_configuration, client_configuration)
    }
}
