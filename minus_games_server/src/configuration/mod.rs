use clap::Parser;
use minus_games_utils::DataFolder;
use minus_games_utils::GamesFolder;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(long, default_value = "127.0.0.1", env)]
    pub ip: String,
    #[arg(long, default_value = "8415", env)]
    pub port: u16,
    #[arg(long, default_value = GamesFolder {}, env)]
    pub games_folder: PathBuf,
    #[arg(long, default_value = DataFolder {}, env)]
    pub data_folder: PathBuf,
    #[arg(long, env)]
    pub cache_folder: Option<PathBuf>,
    #[arg(long)]
    pub config_file: Option<String>,
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Listening on: {}:{}", &self.ip, &self.port)?;
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

        writeln!(f, "Cache Folder: {:?}", self.cache_folder)?;
        write!(
            f,
            "Config File: {}",
            &self.config_file.as_ref().unwrap_or(&String::from("None"))
        )
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
