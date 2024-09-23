use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::get_game_exe_or_exe;
use std::path::Path;

use super::rpgm_utils::{get_rpgm_linux_exe, get_rpgm_name};

#[derive(Copy, Clone)]
pub struct RPGMFunctions {}

impl EngineInfoFunctions for RPGMFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        get_rpgm_name(game_root)
    }

    fn get_linux_exe(&self, game_root: &Path) -> Option<String> {
        get_rpgm_linux_exe(self, game_root)
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        get_game_exe_or_exe(game_root)
    }

    fn get_sync_folders(&self, _: &Path) -> Option<Vec<String>> {
        Some(vec!["$GAME_ROOT/www/save".to_string()])
    }

    fn get_excludes(&self, _: &Path) -> Option<Vec<String>> {
        Some(vec!["config.rpgsave".into()])
    }
}
