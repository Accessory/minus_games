use clap::Parser;
use minus_games_utils::DataFolder;
use minus_games_utils::GamesFolder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(long, default_value = GamesFolder {}, env)]
    pub games_folder: PathBuf,
    #[arg(long, default_value = DataFolder {}, env)]
    pub data_folder: PathBuf,
    #[arg(long, env)]
    pub cache_folder: Option<PathBuf>,
    #[arg(long, short, default_value = "true", env)]
    pub keep_existing_configs: bool,
    #[arg(long, short, default_value = "false", env)]
    pub cleanup_data_folder: bool,
    #[arg(long, short, env)]
    pub filter: Option<String>,
}

impl Configuration {
    pub fn get_cache_file_if_exists(&self, game_folder_name: &str) -> Option<PathBuf> {
        if let Some(cache_file) = self.get_cache_file(game_folder_name) {
            if cache_file.exists() {
                return Some(cache_file);
            }
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

    fn get_json_path_from_game(&self, game_folder_name: &str) -> PathBuf {
        let json_name = format!("{game_folder_name}.json");
        self.data_folder.join(json_name)
    }

    fn get_csv_path_from_game(&self, game_folder_name: &str) -> PathBuf {
        let csv_name = format!("{game_folder_name}.csv");
        self.data_folder.join(csv_name)
    }

    pub fn does_game_infos_exists(&self, game: &str) -> bool {
        self.get_json_path_from_game(game).is_file() && self.get_csv_path_from_game(game).is_file()
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
            write!(f, "Filter: {}", value)?;
        };
        match &self.filter {
            Some(value) => write!(f, " Cleanup data folder: {}", value),
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
