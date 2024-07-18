use std::path::Path;

use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{find_all_exe_files, find_closest_string, get_title_from_parent_folder};

#[derive(Copy, Clone)]
pub struct WolfRPGEditorEngineFunctions {}

impl EngineInfoFunctions for WolfRPGEditorEngineFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        get_title_from_parent_folder(game_root)
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        let name = self.get_game_name(game_root)?;

        if game_root.join("Game.exe").is_file() {
            return Some("Game.exe".into());
        }
        let mut files: Vec<String> = find_all_exe_files(game_root);
        if files.is_empty() {
            None
        } else {
            let idx = find_closest_string(name.as_str(), &files);
            Some(files.remove(idx))
        }
    }

    fn get_sync_folders(&self, _game_root: &Path) -> Option<Vec<String>> {
        Some(vec!["$GAME_ROOT/saves".to_string()])
    }
}
