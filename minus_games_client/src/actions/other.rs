use crate::runtime::{CLIENT, CONFIG};
use std::ffi::OsString;
use std::path::PathBuf;
use tracing::info;

pub async fn list_json() {
    let games = CLIENT.get_games_list().await;
    print!("{}", serde_json::to_string_pretty(&games).unwrap());
}

pub async fn list() {
    let games = CLIENT.get_games_list().await.unwrap_or_default();
    info!("List Games:");
    for game in games {
        info!("{game}");
    }
}

pub fn get_installed_games() -> Vec<String> {
    if !CONFIG.client_games_folder.exists() || !CONFIG.client_folder.exists() {
        return Vec::new();
    }

    let games: Vec<OsString> = CONFIG
        .client_games_folder
        .read_dir()
        .expect("Failed to read game folder")
        .map(|rd| rd.unwrap().path())
        .filter(|i| i.is_dir())
        .map(|i| i.file_name().unwrap().to_os_string())
        .collect();

    let configs: Vec<PathBuf> = CONFIG
        .client_folder
        .read_dir()
        .expect("Failed to read game folder")
        .map(|rd| rd.unwrap().path())
        .filter(|i| {
            i.is_file()
                && i.extension().is_some_and(|f| f == "json")
                && i.file_stem()
                    .is_some_and(|fs| games.contains(&fs.to_os_string()))
        })
        .collect();

    configs
        .into_iter()
        .map(|i| i.file_stem().unwrap().to_str().unwrap().to_string())
        .collect()
}
