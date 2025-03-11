use crate::runtime::{get_client, get_config};
use log::warn;
use std::io::ErrorKind::CrossesDevices;
use tracing::info;

pub async fn list_json() {
    let games = get_client().get_games_list().await;
    print!("{}", serde_json::to_string_pretty(&games).unwrap());
}

pub async fn list() {
    let games = get_client().get_games_list().await.unwrap_or_default();
    info!("List Games:");
    for game in games {
        info!("{game}");
    }
}

pub fn move_additions_header_to_tmp(game: &str) {
    let additions_header_path = get_config().get_game_additions_header_path(game);
    if !additions_header_path.is_file() {
        return;
    }

    let tmp_header_path = get_config().get_game_additions_header_tmp_folder(game);
    if tmp_header_path.is_file() {
        return;
    }

    if let Err(err) = std::fs::create_dir_all(tmp_header_path.parent().unwrap()) {
        warn!("Could not create temporary directory: {err}");
        return;
    }

    match std::fs::rename(&additions_header_path, &tmp_header_path) {
        Ok(_) => {}
        Err(err) => match err.kind() {
            CrossesDevices => {
                if let Err(err) = std::fs::copy(&additions_header_path, &tmp_header_path) {
                    warn!("Could not copy additions header file: {err}");
                }
            }
            _ => {
                warn!(
                    "Failed to move {} to {}",
                    tmp_header_path.display(),
                    additions_header_path.display()
                )
            }
        },
    };
}
