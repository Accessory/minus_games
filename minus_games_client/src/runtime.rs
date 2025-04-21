use crate::configuration::Configuration;
use crate::minus_games_client::MinusGamesClient;
use crate::runtime::MinusGamesClientEvents::{LogInfoMessage, LogInfoStaticMessage};
use crate::utils::get_folders_in_path;
use clap::Parser;
use log::warn;
use minus_games_utils::constants::INFOS;
use std::ffi::OsString;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicU32};
use tokio::sync::OnceCell;
use tracing::debug;

#[derive(Clone, strum::Display)]
pub enum MinusGamesClientEvents {
    StartDownloadingFiles(usize),
    StartDownloadingFile,
    FinishedDownloadingFile,
    FinishedDownloadingFiles,
    LogInfoMessage(String),
    LogInfoStaticMessage(&'static str),
    StartSyncGameFiles,
    FinishedSyncFileInfos,
    FinishedSyncGameFiles,
    CurrentGame(String),
    StartGame(String),
    CloseGame(String),
    DownloadSaves,
    UploadSaves,
    Close,
}

impl From<String> for MinusGamesClientEvents {
    fn from(value: String) -> Self {
        LogInfoMessage(value)
    }
}
impl From<&'static str> for MinusGamesClientEvents {
    fn from(value: &'static str) -> Self {
        LogInfoStaticMessage(value)
    }
}

static SENDER: OnceCell<Option<tokio::sync::mpsc::Sender<MinusGamesClientEvents>>> =
    OnceCell::const_new();

pub async fn set_sender(sender: tokio::sync::mpsc::Sender<MinusGamesClientEvents>) {
    SENDER.get_or_init(|| async move { Some(sender) }).await;
}

pub async fn send_event(event: MinusGamesClientEvents) {
    if let Some(Some(sender)) = SENDER.get() {
        sender.send(event).await.ok();
    }
}

pub static mut CONFIG: Option<Configuration> = None;
pub fn get_config() -> &'static Configuration {
    get_mut_config()
}

pub fn get_mut_config() -> &'static mut Configuration {
    #[allow(static_mut_refs)]
    unsafe {
        CONFIG.get_or_insert_with(Configuration::parse)
    }
}

static mut CLIENT: Option<MinusGamesClient> = None;

pub fn get_client() -> &'static MinusGamesClient {
    unsafe {
        #[allow(static_mut_refs)]
        CLIENT.get_or_insert_with(|| {
            MinusGamesClient::new(
                get_config().server_url.as_str(),
                get_config().username.as_ref(),
                get_config().password.as_ref(),
            )
        })
    }
}

pub async fn download_file_from_to(link: &str, to: &Path) -> bool {
    let mut response = get_client().get(link).await;

    if !response.status().is_success() {
        return false;
    }

    if let Ok(to_write_to) = std::fs::File::create(to) {
        let mut buf_writer = BufWriter::new(to_write_to);
        while let Ok(Some(chunk)) = response.chunk().await {
            if let Err(err) = buf_writer.write(&chunk) {
                warn!("Failed to write to file: {}", err);
            }
        }
    }

    false
}

pub fn reset_client() {
    unsafe {
        #[allow(static_mut_refs)]
        let _ = CLIENT.take();
    }
}

pub static OFFLINE: AtomicBool = AtomicBool::new(false);
pub static SYNC: AtomicBool = AtomicBool::new(true);
pub static SYNC_TESTED: AtomicBool = AtomicBool::new(false);
pub static STOP_DOWNLOAD: AtomicBool = AtomicBool::new(false);

pub async fn get_all_games() -> Vec<String> {
    let mut installed_games = get_installed_games();
    let games = get_client().get_games_list().await.unwrap_or_default();

    for game in games {
        if !installed_games.contains(&game) {
            installed_games.push(game);
        }
    }

    installed_games
}

pub fn get_installed_games() -> Vec<String> {
    if !get_config().client_games_folder.exists() || !get_config().client_folder.exists() {
        return Vec::new();
    }

    let games: Vec<OsString> = get_folders_in_path(&get_config().client_games_folder);

    let configs: Vec<PathBuf> = get_config()
        .client_folder
        .join(INFOS)
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

pub static CURRENT_GAME_PROCESS_ID: AtomicU32 = AtomicU32::new(u32::MAX);

#[cfg(target_family = "windows")]
pub fn kill_current_running_game() {
    let process_id = CURRENT_GAME_PROCESS_ID.load(Relaxed);
    if process_id != u32::MAX {
        debug!("Kill process {process_id}");
        match std::process::Command::new("taskkill")
            .arg("/F")
            .arg("/PID")
            .arg(process_id.to_string())
            .output()
        {
            Ok(_) => {}
            Err(err) => {
                warn!("Failed to execute kill command: {}", err);
            }
        };
    } else {
        warn!("Currently no running game found");
    }
}

#[cfg(not(target_family = "windows"))]
pub fn kill_current_running_game() {
    let process_id = CURRENT_GAME_PROCESS_ID.load(Relaxed);
    if process_id != u32::MAX {
        debug!("Kill process {process_id}");
        match std::process::Command::new("pkill")
            .arg("-P")
            .arg(process_id.to_string())
            .output()
        {
            Ok(_info) => {}
            Err(err) => {
                warn!("Failed to execute kill command: {}", err);
            }
        };
    } else {
        warn!("Currently no running game found");
    }
}

#[macro_export]
macro_rules! offline_to_none {
    () => {
        if OFFLINE.load(Relaxed) {
            debug!("Client is offline!");
            return None;
        }
    };
}

#[macro_export]
macro_rules! offline_to_return {
    () => {
        if OFFLINE.load(Relaxed) {
            debug!("Client is offline!");
            return;
        }
    };
}

#[macro_export]
macro_rules! sync_to_return {
    () => {
        if !SYNC_TESTED.load(Relaxed) {
            SYNC_TESTED.store(true, SeqCst);
            let result = get_client().can_sync().await;
            SYNC.store(result, SeqCst);
        }

        if !SYNC.load(Relaxed) {
            warn!("Client has deactivated the savegame file sync!");
            return;
        }
    };
}
