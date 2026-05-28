use crate::actions::sync::force_sync_all_game_files;
use crate::runtime::{
    MinusGamesClientEvents, STOP_DOWNLOAD, get_config, get_installed_games, send_event,
};
use log::info;
use std::fs::File;
use std::sync::atomic::Ordering::Relaxed;
use tracing::warn;

pub async fn repair_game(game: &str) {
    STOP_DOWNLOAD.store(false, Relaxed);
    send_event(MinusGamesClientEvents::CurrentGame(game.to_string())).await;
    force_sync_all_game_files(game).await;
    send_event(MinusGamesClientEvents::FinishedSyncFileInfos).await;
}

pub async fn check_for_corruption_for_game(game: &str) {
    STOP_DOWNLOAD.store(false, Relaxed);
    send_event(MinusGamesClientEvents::CurrentGame(game.to_string())).await;
    check_game_for_corrupt_files(game).await;
    send_event(MinusGamesClientEvents::FinishedSyncFileInfos).await;
}

async fn check_game_for_corrupt_files(game: &str) {
    info!("Check for corrupted Files for game {game}");
    let game_file_infos = get_config()
        .get_game_file_list(game)
        .expect("Game File List not found");
    for info in game_file_infos {
        let file_path = get_config().client_games_folder.join(info.file_path);

        if !file_path.is_file() {
            warn!(
                "Did not find File: {} with Filesize: {} and Hash: {}",
                file_path.display(),
                info.size,
                info.hash
            );
        }

        let metadata = match file_path.as_path().metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                warn!(
                    "Failed to get the metadata for File: {} with Filesize: {} and Hash: {} with Error: {}",
                    file_path.display(),
                    info.size,
                    info.hash,
                    err
                );
                continue;
            }
        };

        if metadata.len() != info.size {
            warn!(
                "Wrong size for File: {} with Filesize: {} and Hash: {}",
                file_path.display(),
                info.size,
                info.hash,
            );
            continue;
        }
        let file = match File::open(&file_path) {
            Ok(value) => value,
            Err(err) => {
                warn!(
                    "Wrong size for File: {} with Filesize: {}, Hash: {} and Error {}",
                    file_path.display(),
                    info.size,
                    info.hash,
                    err
                );
                continue;
            }
        };
        let file_hash = blake3::Hasher::new()
            .update_reader(file)
            .unwrap()
            .finalize()
            .to_string();
        if file_hash != info.hash {
            warn!(
                "Wrong hash for File: {} with Filesize: {} and Hash: {} while current Filehash: {}",
                file_path.display(),
                info.size,
                info.hash,
                file_hash
            );
            continue;
        }
    }
}

pub async fn repair_all_games() {
    STOP_DOWNLOAD.store(false, Relaxed);
    let games = get_installed_games();
    for game in games {
        send_event(MinusGamesClientEvents::CurrentGame(game.to_string())).await;
        force_sync_all_game_files(&game).await;
    }
    send_event(MinusGamesClientEvents::FinishedSyncFileInfos).await;
}
