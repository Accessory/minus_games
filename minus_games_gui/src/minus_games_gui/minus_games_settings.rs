use crate::minus_games_gui::configuration::GuiConfiguration;
use crate::runtime::get_gui_config;
use iced::Theme;
use minus_games_client::configuration::Configuration;

#[derive(Clone, Debug, Default)]
pub(crate) struct MinusGamesSettings {
    pub server_url: String,
    pub client_folder: String,
    pub client_games_folder: String,
    pub wine_exe: String,
    pub wine_prefix: String,
    pub verbose: bool,
    pub offline: bool,
    pub sync: bool,
    pub fullscreen: bool,
    pub username: String,
    pub password: String,
    pub theme: Option<Theme>,
    pub initial_theme: Option<Theme>,
    pub scale: f32,
    pub font: String,
}

impl MinusGamesSettings {
    pub(crate) fn get_theme_name(&self) -> String {
        if let Some(theme) = self.theme.as_ref() {
            theme.to_string()
        } else {
            "System".to_string()
        }
    }

    pub(crate) fn get_optinal_theme_name(&self) -> Option<String> {
        self.theme.as_ref().map(|theme| theme.to_string())
    }

    pub fn from_config_with_theme(
        value: &Configuration,
        value_gui: &GuiConfiguration,
        theme: Option<Theme>,
    ) -> Self {
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
            sync: value.sync,
            fullscreen: get_gui_config().fullscreen,
            username: value.username.clone().unwrap_or_default(),
            password: value.password.clone().unwrap_or_default(),
            initial_theme: theme.clone(),
            theme,
            scale: value_gui.scale.unwrap_or(1.0),
            font: value_gui.font.clone(),
        }
    }
}
