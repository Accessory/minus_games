use super::sync::{download_sync_for_game, sync_all_game_files, upload_sync_for_game};
use crate::actions::download::download_game;
use crate::runtime::{
    CURRENT_GAME_PROCESS_ID, MinusGamesClientEvents, STOP_DOWNLOAD, get_config, send_event,
};
#[cfg(target_family = "unix")]
use crate::utils::{add_permissions, is_not_executable, make_executable};
#[cfg(target_family = "unix")]
use convert_case::{Case, Casing};
use minus_games_models::game_infos::GameInfos;
use std::env::consts::OS;
use std::error::Error;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
#[cfg(target_family = "unix")]
use std::path::Path;
use std::process::Output;
use std::sync::atomic::Ordering::Relaxed;
use tokio::process::Command;
use tracing::debug;
use tracing::warn;

pub async fn sync_run_game(game: &str) {
    sync_all_game_files(game).await;
    if !get_config().get_game_path(game).is_dir() {
        download_game(game).await;
    }
    run_game(game).await;
}

pub async fn run_game(game: &str) {
    if get_config().is_game_dirty(game) {
        warn!("Game is dirty - repair required!");
        return;
    }

    let infos = match get_config().get_game_infos(game) {
        Some(infos) => infos,
        None => {
            warn!("GameInfos not found for game {game}");
            return;
        }
    };

    send_event(format!("Support: {}", infos.get_supported_platforms()).into()).await;

    get_config().mark_last_time_played(game);

    #[cfg(target_family = "unix")]
    if infos.supports_linux() {
        return run_linux_game_on_linux(infos).await;
    } else if infos.supports_windows() {
        return run_windows_game_on_linux(infos).await;
    }
    #[cfg(target_family = "windows")]
    if infos.supports_windows() {
        return run_windows_game_on_windows(infos).await;
    }
    warn!("Unsupported OS {OS} - {}", infos.get_supported_platforms())
}

pub async fn run_game_synced(game: &str) {
    STOP_DOWNLOAD.store(false, Relaxed);
    send_event(MinusGamesClientEvents::CurrentGame(game.to_string())).await;
    send_event("Sync game files.".into()).await;
    sync_all_game_files(game).await;
    send_event(MinusGamesClientEvents::FinishedSyncGameFiles).await;
    send_event("Download Saves.".into()).await;
    download_sync_for_game(game).await;
    send_event(format!("Run Game {game}").into()).await;
    send_event(MinusGamesClientEvents::StartGame(game.to_string())).await;
    run_game(game).await;
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

    let child = Command::new(path_str)
        .current_dir(&cwd)
        .spawn()
        .expect("Failed to spawn a child process!");

    CURRENT_GAME_PROCESS_ID.store(child.id().expect("Failed to get the process id"), Relaxed);
    handle_command_output(child.wait_with_output().await, &infos.name);
    CURRENT_GAME_PROCESS_ID.store(u32::MAX, Relaxed);
}

#[cfg(target_family = "unix")]
pub async fn run_windows_game_on_linux(infos: GameInfos) {
    if get_config().wine_exe.is_none() || get_config().wine_prefix.is_none() {
        warn!("Cannot run the games since Wine is not configured");
        return;
    }

    let path = infos
        .get_windows_exe(get_config().client_games_folder.as_path())
        .unwrap();
    let path_str = path.as_os_str().to_str().unwrap();
    let prefix = get_config().wine_prefix.as_ref().unwrap().to_str().unwrap();
    let wine = get_config().wine_exe.as_ref().unwrap().to_str().unwrap();
    let cwd = get_config()
        .get_game_path(infos.folder_name.as_str())
        .to_str()
        .unwrap()
        .to_string();
    let game_id = format!("umu-{}", &infos.name).to_case(Case::Kebab);
    let protonpath = if wine.contains("umu") {
        "GE-Proton"
    } else {
        wine
    };

    if has_gamemoderun() {
        send_event("Running game via wine on linux with gamemode".into()).await;
        if get_config().verbose {
            debug!("Running Cmd");
            debug!(
                r#"cd "{}" && WINEPREFIX="{}" PROTONPATH={} GAMEID="{}" gamemoderun "{}" "{}""#,
                cwd, prefix, protonpath, game_id, wine, path_str
            );
        }
        let child = Command::new("gamemoderun")
            .current_dir(&cwd)
            .env("WINEPREFIX", prefix)
            .env("PROTONPATH", protonpath)
            .env("GAMEID", game_id)
            .arg(wine)
            .arg(path_str)
            .spawn()
            .expect("Failed to spawn a child process!");

        CURRENT_GAME_PROCESS_ID.store(child.id().expect("Failed to get the process id"), Relaxed);
        handle_command_output(child.wait_with_output().await, &infos.name);
        CURRENT_GAME_PROCESS_ID.store(u32::MAX, Relaxed);
    } else {
        send_event("Running game via wine on linux without gamemoderun".into()).await;
        if get_config().verbose {
            debug!("Running Cmd");
            debug!(
                r#"cd "{}" && WINEPREFIX="{}" PROTONPATH={} GAMEID="{}" gamemoderun "{}" "{}""#,
                cwd, prefix, protonpath, game_id, wine, path_str
            );
        }

        let child = Command::new(wine)
            .current_dir(&cwd)
            .arg(path_str)
            .env("WINEPREFIX", prefix)
            .env("PROTONPATH", protonpath)
            .env("GAMEID", game_id)
            .spawn()
            .expect("Failed to spawn a child process!");

        CURRENT_GAME_PROCESS_ID.store(child.id().expect("Failed to get the process id"), Relaxed);
        handle_command_output(child.wait_with_output().await, &infos.name);
        CURRENT_GAME_PROCESS_ID.store(u32::MAX, Relaxed);
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

    let child = Command::new(path_str)
        .current_dir(&cwd)
        .spawn()
        .expect("Failed to spawn a child process");

    CURRENT_GAME_PROCESS_ID.store(child.id().expect("Failed to get the process id"), Relaxed);
    handle_command_output(child.wait_with_output().await, &infos.name);
    CURRENT_GAME_PROCESS_ID.store(u32::MAX, Relaxed);
}

fn handle_command_output(output: Result<Output, impl Error>, game: &str) {
    match output {
        Ok(output) => {
            if get_config().verbose {
                debug!(
                    "Stdout:\n{}",
                    String::from_utf8(output.stdout).unwrap_or_default()
                );
                debug!(
                    "Stderr:\n{}",
                    String::from_utf8(output.stderr).unwrap_or_default()
                );
                debug!("Exit with status: {}", output.status);
            }
        }
        Err(err) => {
            warn!(
                "The installation of game '{}' is corrupt. Error: {}.",
                game, err
            );
        }
    }
}
