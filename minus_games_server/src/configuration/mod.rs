use chrono::{DateTime, Utc};
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

impl Configuration {
    pub fn get_game_list(&self) -> Vec<String> {
        let path = self
            .data_folder
            .join("*.json")
            .to_str()
            .unwrap()
            .to_string();
        let mut rtn: Vec<String> = Vec::new();
        for entry in glob::glob(&path).unwrap() {
            rtn.push(
                entry
                    .unwrap()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }
        rtn
    }
    pub fn get_modification_date_for_game(&self, name: &str) -> DateTime<Utc> {
        let path = self.data_folder.join(format!("{name}.json"));
        let system_time = path.metadata().unwrap().modified().unwrap();
        DateTime::<Utc>::from(system_time)
    }

    pub fn does_game_has_header_image(&self, name: &str) -> bool {
        self.data_folder
            .join("additions")
            .join(name)
            .join("header.jpg")
            .is_file()
    }
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
