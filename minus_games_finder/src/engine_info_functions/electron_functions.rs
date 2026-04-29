use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{get_game_exe_or_exe, get_title_from_parent_folder};
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::game_finding_utils::get_linux_exe;

#[derive(Copy, Clone)]
pub struct Electron {}

impl EngineInfoFunctions for Electron {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        get_title_from_parent_folder(game_root)
    }

    fn get_linux_exe(&self, game_root: &Path) -> Option<String> {
        get_linux_exe(self, game_root)
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        get_game_exe_or_exe(game_root)
    }

    fn get_sync_folders(&self, game_root: &Path) -> Option<Vec<String>> {
        let config_path_file = game_root
            .join("resources")
            .join("app")
            .join("public")
            .join("game")
            .join("config.txt");

        if !config_path_file.is_file() {
            return None;
        }

        let config_file_content = std::fs::File::open(config_path_file).ok()?;
        let buf_reader = BufReader::new(config_file_content);
        let lines = buf_reader.lines();

        for line_result in lines {
            let line = line_result.ok()?;
            let line_trimmed = line.trim();
            if line_trimmed.starts_with("Game_name")
                && let Some((_, last_part)) = line_trimmed.split_once(':')
            {
                // TODO: insert when trim_suffix is stable
                // let game_name = last_part.trim_suffix(';');

                let game_name = if last_part.chars().last()? == ';' {
                    &last_part[0..last_part.len() - 1]
                } else {
                    last_part
                };

                return Some(vec![format!(
                    "$APPDATA_ROAMING_OR_CONFIG/{game_name}/IndexedDB"
                )]);
            }
        }

        None
    }

    fn get_excludes(&self, _: &Path) -> Option<Vec<String>> {
        None
    }
}
