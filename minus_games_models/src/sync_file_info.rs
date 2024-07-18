use chrono::{DateTime, Utc};
use minus_games_utils::create_file_list;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub struct SyncFileInfo {
    pub file_name: String,
    pub file_path: String,
    pub size: u64,
    pub last_modified: DateTime<Utc>,
}

impl Display for SyncFileInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "File Name: {}, File Path: {}, Size: {}, Last Modified: {}",
            self.file_name, self.file_path, self.size, self.last_modified
        )
    }
}

impl SyncFileInfo {
    pub fn from_path_with_cut_off(file: PathBuf, cut_off: usize) -> SyncFileInfo {
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let file_path = file
            .iter()
            .skip(cut_off)
            .collect::<PathBuf>()
            .to_str()
            .unwrap()
            .to_string()
            .replace("\\", "/");
        let metadata = file.metadata().unwrap();
        let size = metadata.len();
        let last_modified = metadata.modified().unwrap();
        SyncFileInfo {
            file_name,
            file_path,
            size,
            last_modified: last_modified.into(),
        }
    }
}

pub fn create_sync_file_infos_from_path(folder: &Path) -> Vec<SyncFileInfo> {
    let file_list = create_file_list(folder);
    let mut rtn = Vec::with_capacity(file_list.len());
    let cut_off = std::path::absolute(folder).unwrap().iter().count();
    for file in file_list {
        let sync_file_info = SyncFileInfo::from_path_with_cut_off(file, cut_off);
        rtn.push(sync_file_info);
    }

    rtn
}
