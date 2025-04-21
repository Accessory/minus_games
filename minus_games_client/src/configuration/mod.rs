use crate::utils::{is_or_none, is_or_none_path_buf, is_or_none_string};
use clap::{Parser, Subcommand, command};
use log::warn;
use minus_games_models::game_file_info::GameFileInfo;
use minus_games_models::game_infos::GameInfos;
use minus_games_utils::constants::{ADDITIONS, INFOS};
use minus_games_utils::{ClientFolder, get_last_time_played_path};
use minus_games_utils::{ClientGamesFolder, get_csv_path, get_dirty_path, get_game_infos_path};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::SystemTime;

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
    #[arg(
        long,
        default_value = "http://127.0.0.1:8415",
        env = "MINUS_GAMES_SERVER_URL"
    )]
    pub server_url: String,
    #[arg(long, default_value = ClientFolder {}, env = "MINUS_GAMES_CLIENT_FOLDER" )]
    pub client_folder: PathBuf,
    #[arg(long, env = "MINUS_GAMES_WINE_EXE")]
    pub wine_exe: Option<PathBuf>,
    #[arg(long, env = "MINUS_GAMES_WINE_PREFIX")]
    pub wine_prefix: Option<PathBuf>,
    #[arg(short, long, default_value = "false", env = "MINUS_GAMES_VERBOSE")]
    pub verbose: bool,
    #[arg(short, long, default_value = "false", env = "MINUS_GAMES_OFFLINE")]
    pub offline: bool,
    #[arg(long, default_value = ClientGamesFolder {}, env = "MINUS_GAMES_CLIENT_GAMES_FOLDER")]
    pub client_games_folder: PathBuf,
    #[arg(long, env = "MINUS_GAMES_USERNAME")]
    pub username: Option<String>,
    #[arg(long, env = "MINUS_GAMES_PASSWORD")]
    pub password: Option<String>,
    #[arg(long, default_value = "false", env = "MINUS_GAMES_NO_GAMEMODERUN")]
    pub no_gamemoderun: bool,
    #[arg(long, default_value = "true", env = "MINUS_GAMES_SYNC")]
    pub sync: bool,
    #[command(subcommand)]
    pub action: Option<ClientActions>,
}

impl Configuration {
    pub fn create_necessary_folders(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(self.client_folder.join(INFOS))?;
        std::fs::create_dir_all(self.client_folder.join(ADDITIONS))?;
        std::fs::create_dir_all(self.client_games_folder.as_path())?;
        Ok(())
    }
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

    pub fn get_game_infos(&self, game: &str) -> Option<GameInfos> {
        let json_path = self.get_game_infos_path_from_game(game);
        let file = File::open(json_path).ok()?;
        let buf = BufReader::new(file);
        serde_json::from_reader(buf).ok()
    }

    pub fn get_game_infos_path_from_game(&self, game: &str) -> PathBuf {
        get_game_infos_path(&self.client_folder, game)
    }

    pub fn get_csv_path_for_game(&self, game: &str) -> PathBuf {
        get_csv_path(&self.client_folder, game)
    }

    pub fn get_dirty_path_for_game(&self, game: &str) -> PathBuf {
        get_dirty_path(&self.client_folder, game)
    }

    pub fn get_last_time_played_path_for_game(&self, game: &str) -> PathBuf {
        get_last_time_played_path(&self.client_folder, game)
    }

    pub fn get_game_last_action_date(&self, game: &str) -> SystemTime {
        let game_last_played_path = self.get_last_time_played_path_for_game(game);
        if game_last_played_path.is_file() {
            // info!("Game last played decided by {}", game_last_played_path.display());
            game_last_played_path
                .metadata()
                .unwrap()
                .modified()
                .unwrap()
        } else {
            // info!("Game last played decided by {}", self.get_game_infos_path_from_game(game).display());
            self.get_game_infos_path_from_game(game)
                .metadata()
                .unwrap()
                .modified()
                .unwrap()
        }
    }

    pub fn get_game_file_list(&self, game: &str) -> Option<Vec<GameFileInfo>> {
        let csv_path = self.get_csv_path_for_game(game);
        let mut reader = csv::ReaderBuilder::new().from_path(csv_path).ok()?;
        Some(reader.deserialize().map(|i| i.unwrap()).collect())
    }

    pub fn mark_games_as_dirty(&self, game: &str) {
        let dirty_path = self.get_dirty_path_for_game(game);
        match File::create(&dirty_path) {
            Ok(_) => {}
            Err(err) => {
                warn!(
                    "Could not mark a game as dirty! - Game: {} - Path {} - {}",
                    game,
                    dirty_path.display(),
                    err
                );
            }
        }
    }

    pub fn mark_last_time_played(&self, game: &str) {
        let dirty_path = self.get_last_time_played_path_for_game(game);
        match File::create(&dirty_path) {
            Ok(_) => {}
            Err(err) => {
                warn!(
                    "Could not mark last time played! - Game: {} - Path {} - {}",
                    game,
                    dirty_path.display(),
                    err
                );
            }
        }
    }

    pub fn unmark_games_as_dirty(&self, game: &str) {
        let dirty_path = self.get_dirty_path_for_game(game);
        if dirty_path.is_file() {
            match std::fs::remove_file(dirty_path.as_path()) {
                Ok(_) => {}
                Err(err) => {
                    warn!("Could not unmark a dirty game! - Game: {} - {}", game, err);
                }
            }
        }
    }

    pub fn unmark_last_time_played(&self, game: &str) {
        let last_time_played = self.get_last_time_played_path_for_game(game);
        if last_time_played.is_file() {
            match std::fs::remove_file(last_time_played.as_path()) {
                Ok(_) => {}
                Err(err) => {
                    warn!("Could delete last time played! - Game: {} - {}", game, err);
                }
            }
        }
    }

    pub fn is_game_dirty(&self, game: &str) -> bool {
        let dirty_path = self.get_dirty_path_for_game(game);
        dirty_path.is_file()
    }

    pub fn get_header_option(&self, game: &str) -> Option<PathBuf> {
        let header_path = self.get_game_additions_path(game).join("header.jpg");
        if header_path.exists() {
            Some(header_path)
        } else {
            let tmp_image_path = env::temp_dir()
                .join("minus_games_gui")
                .join(ADDITIONS)
                .join(game)
                .join("header.jpg");
            if tmp_image_path.exists() {
                Some(tmp_image_path)
            } else {
                None
            }
        }
    }

    pub fn get_game_additions_path(&self, game: &str) -> PathBuf {
        self.client_folder.join(ADDITIONS).join(game)
    }

    pub fn get_game_additions_header_path(&self, game: &str) -> PathBuf {
        self.get_game_additions_path(game).join("header.jpg")
    }

    pub fn get_game_additions_tmp_folder(&self, game: &str) -> PathBuf {
        env::temp_dir()
            .join("minus_games_gui")
            .join(ADDITIONS)
            .join(game)
    }

    pub fn get_game_additions_header_tmp_folder(&self, game: &str) -> PathBuf {
        self.get_game_additions_tmp_folder(game).join("header.jpg")
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
        writeln!(f, "Sync: {:?}", &self.sync)?;
        write!(f, "Action: {}", is_or_none(self.action.as_ref()))
    }
}
