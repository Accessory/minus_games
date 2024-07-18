use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameFileInfo {
    pub file_name: String,
    pub file_path: String,
    pub size: u64,
    pub last_modified: DateTime<Utc>,
    pub hash: String,
}

impl GameFileInfo {
    pub fn generate_download_link(&self, base_url: &str) -> String {
        Url::parse(base_url)
            .unwrap()
            .join("/download/")
            .unwrap()
            .join(self.file_path.as_str())
            .unwrap()
            .to_string()
    }

    pub fn csv_headers() -> &'static [&'static str] {
        &[
            "File Name",
            "File Path",
            "File Size",
            "Last Modified",
            "Hash",
        ]
    }
}

impl GameFileInfo {
    pub fn from_path_buf_with_cut_off(file: PathBuf, cut_off: usize) -> GameFileInfo {
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let file_path = file
            .iter()
            .skip(cut_off)
            .collect::<PathBuf>()
            .to_str()
            .unwrap()
            .to_string();
        let metadata = file.metadata().unwrap();
        let size = metadata.len();
        let last_modified = metadata.modified().unwrap();
        let file = File::open(file.as_path()).unwrap();

        let hash = blake3::Hasher::new()
            .update_reader(file)
            .unwrap()
            .finalize()
            .to_string();

        GameFileInfo {
            file_name,
            file_path,
            size,
            last_modified: last_modified.into(),
            hash,
        }
    }
}

impl Display for GameFileInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).expect("Failed to serialize to json")
        )
    }
}
