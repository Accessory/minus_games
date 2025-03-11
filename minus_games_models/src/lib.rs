pub mod game_file_info;
pub mod game_infos;
pub mod game_list;
pub mod rpgm_package;
pub mod sync_file_info;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, strum::Display, Default, Serialize, Deserialize, strum::EnumIter, Copy, Clone)]
pub enum GameEngine {
    RenPy,
    FinalFantasyPixelRemaster,
    RPGMaker,
    RPGMakerMZ,
    Unreal,
    Unity,
    UnityOld,
    WolfRPGEditor,
    Kirikiri,
    #[default]
    Other,
}

#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone)]
pub struct SupportedPlatforms {
    pub windows: bool,
    pub linux: bool,
}

impl Display for SupportedPlatforms {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "windows {}, linux {}", self.windows, self.linux)
    }
}
