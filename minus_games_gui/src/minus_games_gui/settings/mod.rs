use crate::minus_games_gui::minus_games_settings::MinusGamesSettings;
use crate::minus_games_gui::views::settings_view::SettingInput;
use crate::runtime::get_mut_gui_config;
use minus_games_client::runtime::{OFFLINE, get_mut_config};
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
    str::FromStr,
    sync::atomic::Ordering,
};
use tracing::{info, warn};

pub(crate) fn override_config(minus_games_settings_option: Option<&MinusGamesSettings>) {
    if let Some(minus_games_settings) = minus_games_settings_option {
        get_mut_config().server_url = minus_games_settings.server_url.clone();
        get_mut_config().client_folder =
            PathBuf::from_str(&minus_games_settings.client_folder).unwrap_or_default();
        get_mut_config().wine_exe = resolve_path(&minus_games_settings.wine_exe);
        get_mut_config().wine_prefix = resolve_path(&minus_games_settings.wine_prefix);
        get_mut_config().verbose = minus_games_settings.verbose;
        get_mut_config().offline = minus_games_settings.offline;
        OFFLINE.store(minus_games_settings.offline, Ordering::Relaxed);
        get_mut_config().client_games_folder =
            PathBuf::from_str(&minus_games_settings.client_games_folder).unwrap_or_default();
        get_mut_config().username = resolve_string(&minus_games_settings.username);
        get_mut_config().password = resolve_string(&minus_games_settings.password);
        get_mut_gui_config().fullscreen = minus_games_settings.fullscreen;
        get_mut_gui_config().theme = minus_games_settings.theme.to_string();
    }
}
pub(crate) fn override_gui_config(minus_games_settings_option: Option<&MinusGamesSettings>) {
    if let Some(minus_games_settings) = minus_games_settings_option {
        get_mut_gui_config().scale = Some(minus_games_settings.scale);
        get_mut_gui_config().theme = minus_games_settings.theme.to_string();
    }
}

fn resolve_path(value: &str) -> Option<PathBuf> {
    if value.is_empty() {
        None
    } else {
        Some(PathBuf::from_str(value).unwrap_or_default())
    }
}

fn resolve_string(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

pub(crate) fn handle_change_event(
    minus_games_settings_option: Option<&mut MinusGamesSettings>,
    change_input: SettingInput,
) {
    if let Some(minus_games_settings) = minus_games_settings_option {
        match change_input {
            SettingInput::ServerUrl(change) => {
                minus_games_settings.server_url = change.trim().to_string();
            }
            SettingInput::ClientFolder(change) => {
                minus_games_settings.client_folder = change.trim().to_string();
            }
            SettingInput::ClientGamesFolder(change) => {
                minus_games_settings.client_games_folder = change.trim().to_string();
            }
            #[cfg(not(target_family = "windows"))]
            SettingInput::WineExe(change) => {
                minus_games_settings.wine_exe = change.trim().to_string();
            }
            #[cfg(not(target_family = "windows"))]
            SettingInput::WinePrefix(change) => {
                minus_games_settings.wine_prefix = change.trim().to_string();
            }
            SettingInput::Verbose(change) => {
                minus_games_settings.verbose = change;
            }
            SettingInput::Offline(change) => {
                minus_games_settings.offline = change;
            }
            SettingInput::Sync(sync) => {
                minus_games_settings.sync = sync;
            }
            SettingInput::Fullscreen(change) => {
                minus_games_settings.fullscreen = change;
            }
            SettingInput::Username(change) => {
                minus_games_settings.username = change.trim().to_string();
            }
            SettingInput::Password(change) => {
                minus_games_settings.password = change.trim().to_string();
            }
            SettingInput::Theme(theme) => {
                minus_games_settings.theme = theme;
            }
            SettingInput::Scale(scale) => {
                minus_games_settings.scale = scale;
            }
            SettingInput::Font(font) => minus_games_settings.font = font,
        };
    } else {
        warn!("Settings are not set!");
    }
}

pub(crate) fn save_new_settings(settings_option: Option<&MinusGamesSettings>) {
    if let Some(settings) = settings_option {
        if let Some(config_dir) = dirs::config_local_dir() {
            let config_path = config_dir.join("minus_games_gui");
            if std::fs::create_dir_all(&config_path).ok().is_some() {
                let config_file_path = config_path.join("config");
                let file = match std::fs::File::create(&config_file_path) {
                    Ok(file) => file,
                    Err(err) => {
                        warn!(
                            "Failed to save settings at: {} with {err}",
                            config_file_path.display()
                        );
                        return;
                    }
                };
                let mut writer = BufWriter::new(file);

                #[cfg(not(target_family = "windows"))]
                const NEW_LINE: &str = "\n";

                #[cfg(target_family = "windows")]
                const NEW_LINE: &str = "\r\n";

                writer
                    .write_all(
                        format!(
                            "MINUS_GAMES_SERVER_URL=\"{}\"{}",
                            settings.server_url, NEW_LINE
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                writer
                    .write_all(
                        format!(
                            "MINUS_GAMES_CLIENT_FOLDER=\"{}\"{}",
                            settings.client_folder.replace("\\", "\\\\"),
                            NEW_LINE
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                writer
                    .write_all(
                        format!(
                            "MINUS_GAMES_CLIENT_GAMES_FOLDER=\"{}\"{}",
                            settings.client_games_folder.replace("\\", "\\\\"),
                            NEW_LINE
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                writer
                    .write_all(
                        format!("MINUS_GAMES_GUI_THEME=\"{}\"{}", settings.theme, NEW_LINE)
                            .as_bytes(),
                    )
                    .unwrap();
                writer
                    .write_all(
                        format!("MINUS_GAMES_GUI_FONT=\"{}\"{}", settings.font, NEW_LINE)
                            .as_bytes(),
                    )
                    .unwrap();
                if !settings.wine_exe.trim().is_empty() {
                    writer
                        .write_all(
                            format!(
                                "MINUS_GAMES_WINE_EXE=\"{}\"{}",
                                settings.wine_exe.trim(),
                                NEW_LINE
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                }
                if !settings.wine_prefix.trim().is_empty() {
                    writer
                        .write_all(
                            format!(
                                "MINUS_GAMES_WINE_PREFIX=\"{}\"{}",
                                settings.wine_prefix.trim(),
                                NEW_LINE
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                }
                writer
                    .write_all(
                        format!("MINUS_GAMES_VERBOSE=\"{}\"{}", settings.verbose, NEW_LINE)
                            .as_bytes(),
                    )
                    .unwrap();
                writer
                    .write_all(
                        format!("MINUS_GAMES_OFFLINE=\"{}\"{}", settings.offline, NEW_LINE)
                            .as_bytes(),
                    )
                    .unwrap();
                writer
                    .write_all(
                        format!("MINUS_GAMES_SYNC=\"{}\"{}", settings.sync, NEW_LINE).as_bytes(),
                    )
                    .unwrap();
                writer
                    .write_all(
                        format!(
                            "MINUS_GAMES_GUI_FULLSCREEN=\"{}\"{}",
                            settings.fullscreen, NEW_LINE
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                if !settings.username.trim().is_empty() {
                    writer
                        .write_all(
                            format!(
                                "MINUS_GAMES_USERNAME=\"{}\"{}",
                                settings.username.trim(),
                                NEW_LINE
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                }
                if !settings.password.trim().is_empty() {
                    writer
                        .write_all(
                            format!(
                                "MINUS_GAMES_PASSWORD=\"{}\"{}",
                                settings.password.trim(),
                                NEW_LINE
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                }
                writer
                    .write_all(
                        format!("MINUS_GAMES_GUI_SCALE=\"{}\"{}", settings.scale, NEW_LINE)
                            .as_bytes(),
                    )
                    .unwrap();
                info!(
                    "Settings successfully saved at: {}",
                    config_file_path.display()
                );
            };
        }
    } else {
        warn!("Settings are not set and cannot be saved");
    }
}
