use clap::Parser;
use minus_games_utils::DataFolder;
use minus_games_utils::{GamesFolder, get_csv_path, get_game_infos_path};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(long, default_value = GamesFolder {}, env = "MINUS_GAMES_GAMES_FOLDER")]
    pub games_folder: PathBuf,
    #[arg(long, default_value = DataFolder {}, env = "MINUS_GAMES_DATA_FOLDER")]
    pub data_folder: PathBuf,
    #[arg(long, env = "MINUS_GAMES_CACHE_FOLDER")]
    pub cache_folder: Option<PathBuf>,
    #[arg(
        long,
        short,
        default_value = "true",
        env = "MINUS_GAMES_KEEP_EXISTING_CONFIGS"
    )]
    pub keep_existing_configs: bool,
    #[arg(
        long,
        short,
        default_value = "false",
        env = "MINUS_GAMES_CLEANUP_DATA_FOLDER"
    )]
    pub cleanup_data_folder: bool,
    #[arg(long, short, env = "MINUS_GAMES_FILTER")]
    pub filter: Option<String>,
}

impl Configuration {
    pub fn get_cache_file_if_exists(&self, game_folder_name: &str) -> Option<PathBuf> {
        if let Some(cache_file) = self.get_cache_file(game_folder_name)
            && cache_file.exists()
        {
            return Some(cache_file);
        }
        None
    }

    pub fn get_cache_file(&self, game_folder_name: &str) -> Option<PathBuf> {
        if let Some(cache_folder) = &self.cache_folder {
            let cache_json_name = format!("{game_folder_name}.json");
            Some(cache_folder.join(cache_json_name))
        } else {
            None
        }
    }

    fn get_game_infos_path_from_game(&self, game_folder_name: &str) -> PathBuf {
        get_game_infos_path(&self.data_folder, game_folder_name)
    }

    fn get_csv_path_from_game(&self, game_folder_name: &str) -> PathBuf {
        get_csv_path(&self.data_folder, game_folder_name)
    }

    pub fn does_game_infos_exists(&self, game: &str) -> bool {
        self.get_game_infos_path_from_game(game).is_file()
            && self.get_csv_path_from_game(game).is_file()
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Game Folder: {}",
            std::path::absolute(&self.games_folder)
                .unwrap()
                .to_str()
                .unwrap()
        )?;
        writeln!(
            f,
            "Data Folder: {}",
            std::path::absolute(&self.data_folder)
                .unwrap()
                .to_str()
                .unwrap()
        )?;
        writeln!(f, "Keep existing files: {}", self.keep_existing_configs)?;
        if let Some(value) = &self.filter {
            write!(f, "Filter: {value}")?;
        };
        match &self.filter {
            Some(value) => write!(f, " Cleanup data folder: {value}"),
            None => write!(f, ""),
        }
    }
}

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct FileConfiguration {
    #[arg(long)]
    pub ip: Option<String>,
    #[arg(long)]
    pub port: Option<u16>,
    #[arg(long)]
    pub game_folder: Option<String>,
    #[arg(long)]
    pub data_folder: Option<String>,
}
