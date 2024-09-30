use crate::minus_games_gui::minus_games_settings::MinusGamesSettings;
use crate::minus_games_gui::views::settings_view::SettingInput;
use minus_games_client::runtime::{get_mut_config, OFFLINE};
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
    str::FromStr,
    sync::atomic::Ordering,
};
use tracing::{info, warn};
use crate::runtime::get_mut_gui_config;
// unsafe fn change_config_value<T>() {

// }

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
    }
}

// fn resolve_bool_os_str(fullscreen: bool) -> &'static str {
//     if fullscreen {
//         "true"
//     } else {
//         "false"
//     }
// }

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
            SettingInput::WineExe(change) => {
                minus_games_settings.wine_exe = change.trim().to_string();
            }
            SettingInput::WinePrefix(change) => {
                minus_games_settings.wine_prefix = change.trim().to_string();
            }
            SettingInput::Verbose(change) => {
                minus_games_settings.verbose = change;
            }
            SettingInput::Offline(change) => {
                minus_games_settings.offline = change;
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

                writer.write_all(b"SERVER_URL=").unwrap();
                writer.write_all(settings.server_url.as_bytes()).unwrap();
                writer.write_all(NEW_LINE.as_bytes()).unwrap();
                writer.write_all(b"CLIENT_FOLDER=").unwrap();
                writer.write_all(settings.client_folder.as_bytes()).unwrap();
                writer.write_all(NEW_LINE.as_bytes()).unwrap();
                writer.write_all(b"CLIENT_GAMES_FOLDER=").unwrap();
                writer
                    .write_all(settings.client_games_folder.as_bytes())
                    .unwrap();
                writer.write_all(NEW_LINE.as_bytes()).unwrap();
                if !settings.wine_exe.trim().is_empty() {
                    writer.write_all(b"WINE_EXE=").unwrap();
                    writer
                        .write_all(settings.wine_exe.trim().as_bytes())
                        .unwrap();
                    writer.write_all(NEW_LINE.as_bytes()).unwrap();
                }
                if !settings.wine_prefix.trim().is_empty() {
                    writer.write_all(b"WINE_PREFIX=").unwrap();
                    writer
                        .write_all(settings.wine_prefix.trim().as_bytes())
                        .unwrap();
                    writer.write_all(NEW_LINE.as_bytes()).unwrap();
                }
                writer.write_all(b"VERBOSE=").unwrap();
                writer.write_all(resolve_bool(settings.verbose)).unwrap();
                writer.write_all(NEW_LINE.as_bytes()).unwrap();
                writer.write_all(b"OFFLINE=").unwrap();
                writer.write_all(resolve_bool(settings.offline)).unwrap();
                writer.write_all(NEW_LINE.as_bytes()).unwrap();
                writer.write_all(b"MINUS_GAMES_GUI_FULLSCREEN=").unwrap();
                writer.write_all(resolve_bool(settings.fullscreen)).unwrap();
                writer.write_all(NEW_LINE.as_bytes()).unwrap();
                if !settings.username.trim().is_empty() {
                    writer.write_all(b"MINUS_GAMES_USERNAME=").unwrap();
                    writer
                        .write_all(settings.username.trim().as_bytes())
                        .unwrap();
                    writer.write_all(NEW_LINE.as_bytes()).unwrap();
                }
                if !settings.password.trim().is_empty() {
                    writer.write_all(b"MINUS_GAMES_PASSWORD=").unwrap();
                    writer
                        .write_all(settings.password.trim().as_bytes())
                        .unwrap();
                    writer.write_all(NEW_LINE.as_bytes()).unwrap();
                }

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

fn resolve_bool(value: bool) -> &'static [u8] {
    match value {
        true => b"true",
        false => b"false",
    }
}
