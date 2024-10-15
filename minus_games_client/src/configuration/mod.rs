use crate::utils::{
    get_csv_name, get_json_name, is_or_none, is_or_none_path_buf, is_or_none_string,
};
use clap::{command, Parser, Subcommand};
use minus_games_models::game_file_info::GameFileInfo;
use minus_games_models::game_infos::GameInfos;
use minus_games_utils::ClientFolder;
use minus_games_utils::CurrentDir;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, clap::Args, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct DownloadArgs {
    #[arg()]
    pub game: Option<String>,
}

#[derive(Debug, Subcommand, Serialize, Deserialize, strum::Display, Eq, PartialEq, Clone)]
pub enum ClientActions {
    List,
    ListJson,
    Download(DownloadArgs),
    Sync,
    SelectDownload,
    RunGame {
        game: String,
    },
    RunGameSynced {
        game: String,
    },
    SyncRunGame {
        game: String,
    },
    SelectGame,
    DeleteGame {
        game: String,
        purge: Option<bool>,
    },
    SelectDeleteGame {
        purge: Option<bool>,
    },
    Menu,
    Repair {
        game: String,
    },
    SelectRepair,
    DownloadSyncs,
    DownloadSync {
        game: String,
    },
    UploadSyncs,
    ScanForGames,
    #[cfg(target_family = "unix")]
    SelectGameToPlay,
    Gui,
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(long, default_value = "http://127.0.0.1:8415", env)]
    pub server_url: String,
    #[arg(long, default_value = ClientFolder {}, env)]
    pub client_folder: PathBuf,
    #[arg(long, env)]
    pub wine_exe: Option<PathBuf>,
    #[arg(long, env)]
    pub wine_prefix: Option<PathBuf>,
    #[arg(short, long, default_value = "false", env)]
    pub verbose: bool,
    #[arg(short, long, default_value = "false", env)]
    pub offline: bool,
    #[arg(long, default_value = CurrentDir {}, env)]
    pub client_games_folder: PathBuf,
    #[arg(long, env = "MINUS_GAMES_USERNAME")]
    pub username: Option<String>,
    #[arg(long, env = "MINUS_GAMES_PASSWORD")]
    pub password: Option<String>,
    #[command(subcommand)]
    pub action: Option<ClientActions>,
}

impl Configuration {
    pub fn get_game_path(&self, game: &str) -> PathBuf {
        self.client_games_folder.join(game)
    }

    pub fn get_game_info(&self, game: &str) -> Option<Vec<GameFileInfo>> {
        let csv_path = self.get_csv_path_for_game(game);
        let csv_file = File::open(csv_path.as_path()).ok()?;
        let csv_buf_reader = BufReader::new(csv_file);
        let mut reader = csv::ReaderBuilder::new().from_reader(csv_buf_reader);
        Some(reader.deserialize().map(|i| i.unwrap()).collect())
    }

    pub fn get_json_path(&self, json_name: &str) -> PathBuf {
        self.client_folder.join(json_name)
    }
    pub fn get_game_infos(&self, game: &str) -> Option<GameInfos> {
        let json_path = self.get_json_path_from_game(game);
        let file = File::open(json_path).ok()?;
        let buf = BufReader::new(file);
        serde_json::from_reader(buf).ok()
    }

    pub fn get_json_path_from_game(&self, game: &str) -> PathBuf {
        let json_name = get_json_name(game);
        self.client_folder.join(json_name)
    }

    pub fn get_csv_path_for_game(&self, game: &str) -> PathBuf {
        let csv_name = get_csv_name(game);
        self.client_folder.join(csv_name)
    }

    pub fn get_csv_path(&self, csv_name: &str) -> PathBuf {
        self.client_folder.join(csv_name)
    }

    pub fn get_game_file_list(&self, game: &str) -> Option<Vec<GameFileInfo>> {
        let csv_path = self.get_csv_path_for_game(game);
        let mut reader = csv::ReaderBuilder::new().from_path(csv_path).ok()?;
        Some(reader.deserialize().map(|i| i.unwrap()).collect())
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Server Url: {}", self.server_url.as_str())?;
        writeln!(f, "Client Folder: {}", self.client_folder.display())?;
        writeln!(f, "Game Folder: {}", self.client_games_folder.display())?;
        writeln!(f, "Wine Exe: {}", is_or_none_path_buf(&self.wine_exe))?;
        writeln!(f, "Wine Prefix: {}", is_or_none_path_buf(&self.wine_prefix))?;
        writeln!(f, "Username: {}", is_or_none_string(&self.username))?;
        writeln!(f, "Offline: {:?}", &self.offline)?;
        write!(f, "Action: {}", is_or_none(self.action.as_ref()))
    }
}
