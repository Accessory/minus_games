use crate::{GameEngine, SupportedPlatforms};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Default, Debug, Clone, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct GameInfos {
    pub name: String,
    pub folder_name: String,
    pub engine: GameEngine,
    // pub supported_platforms: SupportedPlatforms,
    pub linux_exe: Option<String>,
    pub windows_exe: Option<String>,
    pub sync_folders: Option<Vec<String>>,
    pub excludes: Option<Vec<String>>,
    // pub additions: Option<Additions>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct MinimalGameInfos {
    pub name: String,
    pub engine: String,
    pub linux: bool,
    pub windows: bool,
}

impl From<GameInfos> for MinimalGameInfos {
    fn from(value: GameInfos) -> Self {
        MinimalGameInfos {
            name: value.folder_name,
            engine: value.engine.to_string(),
            linux: value.linux_exe.is_some(),
            windows: value.windows_exe.is_some(),
        }
    }
}

// #[derive(Serialize, Deserialize, Default, Debug, Clone)]
// #[serde(rename_all = "PascalCase")]
// pub struct Additions {
//     pub header: bool,
// }

impl GameInfos {
    pub fn supports_linux(&self) -> bool {
        self.linux_exe.is_some()
    }

    pub fn supports_windows(&self) -> bool {
        self.windows_exe.is_some()
    }

    pub fn get_supported_platforms(&self) -> SupportedPlatforms {
        SupportedPlatforms {
            windows: self.windows_exe.is_some(),
            linux: self.linux_exe.is_some(),
        }
    }
    pub fn is_excluded(&self, file_path: &str) -> bool {
        if let Some(excludes) = self.excludes.as_ref() {
            for exclude in excludes {
                if file_path.contains(exclude) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_linux_exe(&self, game_folder: &Path) -> Option<PathBuf> {
        std::path::absolute(
            game_folder
                .join(self.folder_name.as_str())
                .join(self.linux_exe.as_ref()?),
        )
        .ok()
    }
    pub fn get_windows_exe(&self, game_folder: &Path) -> Option<PathBuf> {
        std::path::absolute(
            game_folder
                .join(self.folder_name.as_str())
                .join(self.windows_exe.as_ref()?),
        )
        .ok()
    }
}

impl Display for GameInfos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).expect("Failed to serialize to json")
        )
    }
}
