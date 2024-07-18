use std::path::Path;

use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{get_closest_windows_exe, get_title_from_parent_folder};

#[derive(Copy, Clone)]
pub struct KirikiriFunctions {}

impl EngineInfoFunctions for KirikiriFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        get_title_from_parent_folder(game_root)
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        let name = self.get_game_name(game_root)?;
        get_closest_windows_exe(name.as_str(), game_root)
    }

    fn get_sync_folders(&self, _game_root: &Path) -> Option<Vec<String>> {
        Some(vec!["$GAME_ROOT/savedata".to_string()])
    }
}
