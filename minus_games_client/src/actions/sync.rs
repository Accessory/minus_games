use super::download::download_all_files;
use crate::actions::delete::delete_game_info_files;
use crate::actions::other::get_installed_games;
use crate::runtime::{CLIENT, CONFIG};
use chrono::{DateTime, Utc};
use minus_games_models::game_infos::GameInfos;
use minus_games_models::sync_file_info::SyncFileInfo;
use minus_games_utils::{create_file_list, create_hash_from_string, set_file_modified_time};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tracing::{debug, trace, warn};

pub async fn sync_game_infos(game: &str) {
    let json = CONFIG.get_json_path_from_game(game);
    if json.exists() {
        std::fs::remove_file(json).unwrap();
    }

    let csv = CONFIG.get_csv_path_for_game(game);
    if csv.exists() {
        std::fs::remove_file(csv).unwrap();
    }

    CLIENT.download_infos(game).await;
}

pub async fn sync_infos_for_all_games() {
    let games = CLIENT.get_games_list().await.unwrap_or_default();

    for game in games {
        delete_game_info_files(game.as_str());
        sync_game_infos(game.as_str()).await
    }
}

pub async fn sync_all_game_files(game: &str) {
    let has_new_game_infos = CLIENT.download_game_infos_if_modified(game).await;
    let has_new_game_files = CLIENT.download_game_files_if_modified(game).await;

    if has_new_game_files || has_new_game_infos {
        sync_game_files_and_download(game).await;
    }
}

async fn sync_game_files_and_download(game: &str) {
    let game_file_infos = CONFIG
        .get_game_file_list(game)
        .expect("Game File List not found");
    for info in game_file_infos {
        let file_path = CONFIG.client_games_folder.join(info.file_path);
        if let Ok(metadata) = file_path.as_path().metadata() {
            if metadata.len() != info.size && file_path.as_path().is_file() {
                std::fs::remove_file(file_path).unwrap();
            }
        }
    }
    download_all_files(game).await;
}

pub async fn download_sync_for_game(game: &str) {
    let game_infos = match CONFIG.get_game_infos(game) {
        Some(infos) => infos,
        None => {
            warn!("GameInfos not found for game {game}");
            return;
        }
    };

    if let Some(sync_folders) = &game_infos.sync_folders {
        for folder in sync_folders {
            let folder_hash = create_hash_from_string(folder);
            if let Some(file_infos) = CLIENT.get_sync_file_list(game, folder_hash.as_str()).await {
                let sync_path: PathBuf = resolve_sync_path(folder, &game_infos);
                for file_info in file_infos {
                    let download_file_path = sync_path.join(file_info.file_path.as_str());

                    if !download_necessary(download_file_path.as_path(), file_info.last_modified) {
                        continue;
                    }
                    debug!("Downloading: {}", file_info);
                    CLIENT
                        .download_sync_file(
                            game,
                            &folder_hash,
                            file_info.file_path.as_str(),
                            download_file_path.as_path(),
                        )
                        .await;
                    set_file_modified_time(
                        download_file_path.as_path(),
                        file_info.last_modified.into(),
                    );
                }
            }
        }
    }
}

#[inline]
#[cfg(not(target_family = "windows"))]
fn check_if_is_wine(game_infos: &GameInfos) -> bool {
    !game_infos.supported_platforms.linux
}

fn download_necessary(path: &Path, last_modified: DateTime<Utc>) -> bool {
    !path.is_file()
        || path.metadata().unwrap().modified().unwrap() != Into::<SystemTime>::into(last_modified)
}

pub async fn upload_sync_for_game(game: &str) {
    let game_infos = match CONFIG.get_game_infos(game) {
        Some(infos) => infos,
        None => {
            warn!("GameInfos not found for game {game}");
            return;
        }
    };

    if let Some(sync_folders) = &game_infos.sync_folders {
        for folder in sync_folders {
            let folder_hash = create_hash_from_string(folder);
            let sync_path: PathBuf = resolve_sync_path(folder, &game_infos);
            let sfi_server = CLIENT.get_sync_file_list(game, &folder_hash).await;
            let file_list = create_file_list(&sync_path);
            let absolute_path = std::path::absolute(sync_path).unwrap();
            let cut_off = absolute_path.iter().count();
            trace!("Syncing: {} - {}", folder, absolute_path.display());
            for file in file_list {
                let sfi = SyncFileInfo::from_path_with_cut_off(file, cut_off);
                if game_infos.is_excluded(sfi.file_path.as_str()) {
                    continue;
                }

                let upload_file_path = absolute_path.join(sfi.file_path.as_str());

                if !needs_upload(&sfi, &sfi_server) {
                    continue;
                }

                debug!("Uploading: {}", sfi);
                CLIENT
                    .upload_sync_file(game, folder_hash.as_str(), sfi, upload_file_path)
                    .await;
            }
        }
    }
}

fn needs_upload(sfi: &SyncFileInfo, sfi_server_list: &Option<Vec<SyncFileInfo>>) -> bool {
    if sfi_server_list.is_none() {
        return true;
    }

    let list_ref = sfi_server_list.as_ref().unwrap();

    let item = list_ref
        .iter()
        .find(|ssfi| sfi.file_path.as_str() == ssfi.file_path.as_str());

    if item.is_none() {
        return true;
    }

    item.unwrap().last_modified != sfi.last_modified
}

fn resolve_sync_path(to_resolve: &str, game_infos: &GameInfos) -> PathBuf {
    let mut rtn = PathBuf::new();
    let resolve_path = Path::new(to_resolve);

    for part in resolve_path {
        let part_str = part.to_str().unwrap();
        match part_str {
            "$GAME_ROOT" => rtn.push(CONFIG.get_game_path(game_infos.folder_name.as_str())),
            #[cfg(target_family = "windows")]
            "$UNITY_CONFIG" => {
                if let Some(value) = get_local_low() {
                    rtn.push(value);
                }
            }
            #[cfg(target_family = "unix")]
            "$UNITY_CONFIG" => {
                if let Some(value) = resolve_unity_config_path(game_infos) {
                    rtn.push(value);
                }
            }
            "$UNREAL_CONFIG" => {
                if let Some(value) = resolve_unreal_config_path() {
                    rtn.push(value);
                }
            }
            "$DOCUMENTS" => {
                if let Some(value) = resolve_documents_path(game_infos) {
                    rtn.push(value);
                }
            }
            _ => rtn.push(part_str),
        }
    }

    rtn
}

#[cfg(target_family = "windows")]
fn get_local_low() -> Option<PathBuf> {
    let local_appdata = std::env::var("LOCALAPPDATA").ok()?;
    let rtn = Path::new(&local_appdata).parent()?.join("LocalLow");
    Some(rtn)
}

#[cfg(target_family = "unix")]
fn resolve_unity_config_path(game_infos: &GameInfos) -> Option<PathBuf> {
    let is_wine = check_if_is_wine(game_infos);
    if is_wine {
        let wine_prefix = CONFIG.wine_prefix.as_ref()?;
        let user = std::env::var("USER").ok()?;
        let rtn = wine_prefix
            .join("drive_c")
            .join("users")
            .join(user)
            .join("AppData")
            .join("LocalLow");
        Some(rtn)
    } else {
        let mut rtn = match std::env::var("XDG_CONFIG_HOME") {
            Ok(config) => PathBuf::from(config),
            Err(_) => {
                let home = std::env::var("HOME").unwrap_or(".".to_string());
                std::path::absolute(format!("{}/.config", home)).ok()?
            }
        };
        trace!("Config: {}", rtn.display());
        rtn.push("unity3d");
        Some(rtn)
    }
}

#[cfg(target_family = "unix")]
fn resolve_documents_path(game_infos: &GameInfos) -> Option<PathBuf> {
    let is_wine = check_if_is_wine(game_infos);
    if is_wine {
        let wine_prefix = CONFIG.wine_prefix.as_ref()?;
        let user = std::env::var("USER").ok()?;
        let rtn = wine_prefix
            .join("drive_c")
            .join("users")
            .join(user)
            .join("Documents");
        Some(rtn)
    } else {
        let rtn = match dirs::document_dir() {
            Some(path) => path,
            None => {
                let user = std::env::var("USER").ok()?;
                Path::new("home").join(user).join("Documents")
            }
        };
        Some(rtn)
    }
}

#[cfg(target_family = "windows")]
fn resolve_documents_path(_: &GameInfos) -> Option<PathBuf> {
    dirs::document_dir()
}

#[cfg(target_family = "unix")]
fn resolve_unreal_config_path() -> Option<PathBuf> {
    let wine_prefix = CONFIG.wine_prefix.as_ref()?;
    let user = std::env::var("USER").ok()?;
    Some(
        wine_prefix
            .join("drive_c")
            .join("users")
            .join(user)
            .join("AppData")
            .join("Local"),
    )
}

#[cfg(target_family = "windows")]
fn resolve_unreal_config_path() -> Option<PathBuf> {
    let local_low = get_local_low()?;
    Some(local_low.parent()?.join("Local"))
}

pub async fn download_syncs() {
    let installed_games = get_installed_games();

    for game in installed_games.iter() {
        download_sync_for_game(game).await;
    }
}

pub async fn upload_syncs() {
    let installed_games = get_installed_games();

    for game in installed_games.iter() {
        upload_sync_for_game(game).await;
    }
}
