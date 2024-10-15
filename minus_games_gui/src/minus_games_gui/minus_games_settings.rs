use crate::runtime::get_gui_config;
use iced::Theme;
use minus_games_client::configuration::Configuration;

#[derive(Clone, Default, Debug)]
pub(crate) struct MinusGamesSettings {
    pub server_url: String,
    pub client_folder: String,
    pub client_games_folder: String,
    pub wine_exe: String,
    pub wine_prefix: String,
    pub verbose: bool,
    pub offline: bool,
    pub fullscreen: bool,
    pub username: String,
    pub password: String,
    pub theme: Theme,
    pub initial_theme: Theme,
}

impl MinusGamesSettings {
    pub fn from_config_with_theme(value: &Configuration, theme: Theme) -> Self {
        Self {
            server_url: value.server_url.to_string(),
            client_folder: value.client_folder.to_str().unwrap().to_string(),
            client_games_folder: value.client_games_folder.to_str().unwrap().to_string(),
            wine_exe: match value.wine_exe.as_ref() {
                None => "".to_string(),
                Some(val) => val.to_str().unwrap().to_string(),
            },
            wine_prefix: match value.wine_prefix.as_ref() {
                None => "".to_string(),
                Some(val) => val.to_str().unwrap().to_string(),
            },
            verbose: value.verbose,
            offline: value.offline,
            fullscreen: get_gui_config().fullscreen,
            username: value.username.clone().unwrap_or_default(),
            password: value.password.clone().unwrap_or_default(),
            initial_theme: theme.clone(),
            theme,
        }
    }
}
