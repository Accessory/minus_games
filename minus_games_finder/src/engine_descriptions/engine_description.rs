use minus_games_models::GameEngine;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Platform {
    #[default]
    Windows,
    Linux,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PlatformDescription {
    pub platform: Platform,
    pub look_for_files: Vec<String>,
    pub look_for_folders: Vec<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct EngineDescription {
    pub engine_type: GameEngine,
    pub main_files: Vec<String>,
    pub main_folders: Vec<String>,
    pub platform_windows: Option<PlatformDescription>,
    pub platform_linux: Option<PlatformDescription>,
}

impl std::fmt::Display for EngineDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game Engine: {}", self.engine_type)?;
        writeln!(f, "Files:")?;
        for file in self.main_files.iter() {
            writeln!(f, "{file}")?;
        }

        writeln!(f, "Folders:")?;
        for folder in self.main_folders.iter() {
            writeln!(f, "{folder}")?;
        }
        Ok(())
    }
}
