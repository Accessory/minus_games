use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{find_possible_save_dir_in_game_root, get_game_exe_or_exe};
use std::path::Path;

use super::game_finding_utils::{get_linux_exe, get_rpgm_name};

#[derive(Copy, Clone)]
pub struct RPGMFunctions {}

impl EngineInfoFunctions for RPGMFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        get_rpgm_name(game_root)
    }

    fn get_linux_exe(&self, game_root: &Path) -> Option<String> {
        get_linux_exe(self, game_root)
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        get_game_exe_or_exe(game_root)
    }

    fn get_sync_folders(&self, game_root: &Path) -> Option<Vec<String>> {
        find_possible_save_dir_in_game_root(game_root)
    }

    fn get_excludes(&self, _: &Path) -> Option<Vec<String>> {
        Some(vec!["config.rpgsave".into()])
    }
}
