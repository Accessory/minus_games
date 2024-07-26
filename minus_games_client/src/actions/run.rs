use super::sync::{download_sync_for_game, sync_all_game_files, upload_sync_for_game};
use crate::actions::download::download_game;
use crate::actions::sync::sync_game_infos;
use crate::runtime::CONFIG;
#[cfg(target_family = "unix")]
use crate::utils::{add_permissions, is_not_executable, make_executable};
use minus_games_models::game_infos::GameInfos;
use std::env::consts::OS;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
#[cfg(target_family = "unix")]
use std::path::Path;
use std::process::Command;
use tracing::{info, warn};

pub async fn sync_run_game(game: &str) {
    sync_game_infos(game).await;
    if !CONFIG.get_game_path(game).is_dir() {
        download_game(game).await;
    }
    run_game(game);
}

pub fn run_game(game: &str) {
    let infos = match CONFIG.get_game_infos(game) {
        Some(infos) => infos,
        None => {
            warn!("GameInfos not found for game {game}");
            return;
        }
    };
    info!("Support: {}", infos.supported_platforms);

    #[cfg(target_family = "unix")]
    if infos.supported_platforms.linux {
        return run_linux_game_on_linux(infos);
    } else if infos.supported_platforms.windows {
        return run_windows_game_on_linux(infos);
    }
    #[cfg(target_family = "windows")]
    if infos.supported_platforms.windows {
        return run_windows_game_on_windows(infos);
    }
    warn!("Unsupported OS {OS} - {}", infos.supported_platforms)
}

pub async fn run_game_synced(game: &str) {
    info!("Sync game files.");
    sync_all_game_files(game).await;
    info!("Download Saves.");
    download_sync_for_game(game).await;
    info!("Run Game {game}");
    run_game(game);
    info!("Uplaod Saves.");
    upload_sync_for_game(game).await;
}

#[cfg(target_family = "windows")]
pub fn run_windows_game_on_windows(infos: GameInfos) {
    info!("Running game native on windows");
    let path = CONFIG
        .client_games_folder
        .join(infos.folder_name.as_str())
        .join(infos.windows_exe.unwrap().as_str());
    let path_str = path.as_os_str().to_str().unwrap();
    let cwd = CONFIG
        .get_game_path(infos.folder_name.as_str())
        .to_str()
        .unwrap()
        .to_string();
    Command::new(path_str)
        .current_dir(&cwd)
        .output()
        .unwrap_or_else(|err| panic!("Failed to run {path_str} - {err}"));
}

#[cfg(target_family = "unix")]
pub fn run_windows_game_on_linux(infos: GameInfos) {
    info!("Running game via wine on linux");
    if CONFIG.wine_exe.is_none() || CONFIG.wine_prefix.is_none() {
        panic!("Wine not configured");
    }

    if has_gamemoderun() {
        let path = infos
            .get_windows_exe(CONFIG.client_games_folder.as_path())
            .unwrap();
        let path_str = path.as_os_str().to_str().unwrap();
        let prefix = CONFIG.wine_prefix.as_ref().unwrap().to_str().unwrap();
        let exe = CONFIG.wine_exe.as_ref().unwrap().to_str().unwrap();
        let cwd = CONFIG
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
            .unwrap_or_else(|_| panic!("Failed to run {path_str}"));
    } else {
        let path = infos
            .get_windows_exe(CONFIG.client_games_folder.as_path())
            .unwrap();
        let path_str = path.as_os_str().to_str().unwrap();
        let prefix = CONFIG.wine_prefix.as_ref().unwrap().to_str().unwrap();
        let exe = CONFIG.wine_exe.as_ref().unwrap().to_str().unwrap();
        let cwd = CONFIG
            .get_game_path(infos.folder_name.as_str())
            .to_str()
            .unwrap()
            .to_string();
        Command::new(exe)
            .current_dir(&cwd)
            .arg(path_str)
            .env("WINEPREFIX", prefix)
            .output()
            .unwrap_or_else(|_| panic!("Failed to run {path_str}"));
    }
}

#[cfg(target_family = "unix")]
fn has_gamemoderun() -> bool {
    if let Some(path) = std::env::var_os("PATH") {
        let paths = std::env::split_paths(&path);
        for path in paths {
            if path.join("gamemoderun").exists() {
                info!("Use gamemoderun");
                return true;
            }
        }
    }
    info!("gamemoderun not found");
    false
}

#[cfg(target_family = "unix")]
pub fn run_linux_game_on_linux(infos: GameInfos) {
    info!("Running game native on linux");

    let path = infos
        .get_linux_exe(CONFIG.client_games_folder.as_path())
        .unwrap();
    let path_str = path.as_os_str().to_str().unwrap();

    let cwd = CONFIG
        .get_game_path(infos.folder_name.as_str())
        .to_str()
        .unwrap()
        .to_string();

    let mode = path.metadata().unwrap().permissions().mode();
    let exe_stem = path.file_stem().unwrap();
    if is_not_executable(mode) {
        make_executable(path.as_path(), mode);
        let cwd_path = Path::new(cwd.as_str());
        add_permissions(cwd_path, exe_stem);
    }

    match Command::new(path_str).current_dir(&cwd).output() {
        Ok(_) => {}
        Err(err) => panic!("Failed to run {path_str} with error: {}", err),
    };
}
