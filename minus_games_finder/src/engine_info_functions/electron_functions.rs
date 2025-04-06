use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{get_game_exe_or_exe, get_title_from_parent_folder};
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

    fn get_sync_folders(&self, _: &Path) -> Option<Vec<String>> {
        None
    }

    fn get_excludes(&self, _: &Path) -> Option<Vec<String>> {
        None
    }
}
