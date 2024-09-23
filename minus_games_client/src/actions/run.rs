use super::sync::{download_sync_for_game, sync_all_game_files, upload_sync_for_game};
use crate::actions::download::download_game;
use crate::actions::sync::sync_game_infos;
use crate::runtime::{get_config, send_event, MinusGamesClientEvents};
#[cfg(target_family = "unix")]
use crate::utils::{add_permissions, is_not_executable, make_executable};
use minus_games_models::game_infos::GameInfos;
use std::env::consts::OS;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
#[cfg(target_family = "unix")]
use std::path::Path;
use tokio::process::Command;
#[cfg(target_family = "unix")]
use tracing::debug;
use tracing::warn;

pub async fn sync_run_game(game: &str) {
    sync_game_infos(game).await;
    if !get_config().get_game_path(game).is_dir() {
        download_game(game).await;
    }
    run_game(game).await;
}

pub async fn run_game(game: &str) {
    let infos = match get_config().get_game_infos(game) {
        Some(infos) => infos,
        None => {
            warn!("GameInfos not found for game {game}");
            return;
        }
    };

    send_event(format!("Support: {}", infos.supported_platforms).into()).await;

    #[cfg(target_family = "unix")]
    if infos.supported_platforms.linux {
        return run_linux_game_on_linux(infos).await;
    } else if infos.supported_platforms.windows {
        return run_windows_game_on_linux(infos).await;
    }
    #[cfg(target_family = "windows")]
    if infos.supported_platforms.windows {
        return run_windows_game_on_windows(infos).await;
    }
    warn!("Unsupported OS {OS} - {}", infos.supported_platforms)
}

pub async fn run_game_synced(game: &str) {
    send_event(MinusGamesClientEvents::RunningGame(game.to_string())).await;
    send_event("Sync game files.".into()).await;
    sync_all_game_files(game).await;
    send_event(MinusGamesClientEvents::FinishedSyncGameFiles).await;
    send_event("Download Saves.".into()).await;
    download_sync_for_game(game).await;
    send_event(format!("Run Game {game}").into()).await;
    send_event(MinusGamesClientEvents::StartGame(game.to_string())).await;
    let run_game_copy = game.to_string();
    run_game(&run_game_copy).await;
    send_event(MinusGamesClientEvents::CloseGame(game.to_string())).await;
    send_event("Upload Saves.".into()).await;
    upload_sync_for_game(game).await;
}

#[cfg(target_family = "windows")]
pub async fn run_windows_game_on_windows(infos: GameInfos) {
    use crate::runtime::get_config;

    send_event("Running game native on windows".into()).await;
    let path = get_config()
        .client_games_folder
        .join(infos.folder_name.as_str())
        .join(infos.windows_exe.unwrap().as_str());
    let path_str = path.as_os_str().to_str().unwrap();
    let cwd = get_config()
        .get_game_path(infos.folder_name.as_str())
        .to_str()
        .unwrap()
        .to_string();
    Command::new(path_str)
        .current_dir(&cwd)
        .output()
        .await
        .unwrap_or_else(|err| panic!("Failed to run {path_str} - {err}"));
}

#[cfg(target_family = "unix")]
pub async fn run_windows_game_on_linux(infos: GameInfos) {
    if get_config().wine_exe.is_none() || get_config().wine_prefix.is_none() {
        panic!("Wine not configured");
    }

    if has_gamemoderun() {
        send_event("Running game via wine on linux with gamemode".into()).await;
        let path = infos
            .get_windows_exe(get_config().client_games_folder.as_path())
            .unwrap();
        let path_str = path.as_os_str().to_str().unwrap();
        let prefix = get_config().wine_prefix.as_ref().unwrap().to_str().unwrap();
        let exe = get_config().wine_exe.as_ref().unwrap().to_str().unwrap();
        let cwd = get_config()
            .get_game_path(infos.folder_name.as_str())
            .to_str()
            .unwrap()
            .to_string();
        Command::new("gamemoderun")
            .current_dir(&cwd)
            .arg(exe)
            .arg(path_str)
            .env("WINEPREFIX", prefix)
            .output()
            .await
            .unwrap_or_else(|_| panic!("Failed to run {path_str}"));
    } else {
        send_event("Running game via wine on linux without gamemode".into()).await;
        let path = infos
            .get_windows_exe(get_config().client_games_folder.as_path())
            .unwrap();
        let path_str = path.as_os_str().to_str().unwrap();
        let prefix = get_config().wine_prefix.as_ref().unwrap().to_str().unwrap();
        let exe = get_config().wine_exe.as_ref().unwrap().to_str().unwrap();
        let cwd = get_config()
            .get_game_path(infos.folder_name.as_str())
            .to_str()
            .unwrap()
            .to_string();
        Command::new(exe)
            .current_dir(&cwd)
            .arg(path_str)
            .env("WINEPREFIX", prefix)
            .output()
            .await
            .unwrap_or_else(|_| panic!("Failed to run {path_str}"));
    }
}

#[cfg(target_family = "unix")]
fn has_gamemoderun() -> bool {
    if let Some(path) = std::env::var_os("PATH") {
        let paths = std::env::split_paths(&path);
        for path in paths {
            if path.join("gamemoderun").exists() {
                debug!("Use gamemoderun");
                return true;
            }
        }
    }
    debug!("gamemoderun not found");
    false
}

#[cfg(target_family = "unix")]
pub async fn run_linux_game_on_linux(infos: GameInfos) {
    send_event("Running game native on linux".into()).await;
    let path = infos
        .get_linux_exe(get_config().client_games_folder.as_path())
        .unwrap();
    let path_str = path.as_os_str().to_str().unwrap();

    let cwd = get_config()
        .get_game_path(infos.folder_name.as_str())
        .to_str()
        .unwrap()
        .to_string();

    let mode = match path.metadata() {
        Ok(metadata) => metadata.permissions().mode(),
        Err(err) => {
            warn!(
                "The game installation of '{}' is corrupt. Error: {}.",
                infos.name, err
            );
            return;
        }
    };
    let exe_stem = path.file_stem().unwrap();
    if is_not_executable(mode) {
        make_executable(path.as_path(), mode);
        let cwd_path = Path::new(cwd.as_str());
        add_permissions(cwd_path, exe_stem);
    }

    match Command::new(path_str).current_dir(&cwd).output().await {
        Ok(_) => {}
        Err(err) => panic!("Failed to run {path_str} with error: {}", err),
    };
}
