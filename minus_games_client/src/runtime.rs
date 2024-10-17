use crate::configuration::Configuration;
use crate::minus_games_client::MinusGamesClient;
use crate::runtime::MinusGamesClientEvents::{LogInfoMessage, LogInfoStaticMessage};
use crate::utils::get_folders_in_path;
use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use tokio::sync::OnceCell;

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

pub fn reset_client() {
    unsafe {
        #[allow(static_mut_refs)]
        let _ = CLIENT.take();
    }
}

pub static OFFLINE: AtomicBool = AtomicBool::new(false);

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
