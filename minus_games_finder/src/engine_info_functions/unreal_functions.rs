use std::path::Path;

use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{
    get_all_folder_names, get_closest_exe_from_folder, get_title_from_parent_folder,
};

#[derive(Copy, Clone)]
pub struct UnrealFunctions {}

impl EngineInfoFunctions for UnrealFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        get_title_from_parent_folder(game_root)
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        let name = self.get_game_name(game_root)?;
        get_closest_exe_from_folder(game_root, name.as_str())
    }

    fn get_sync_folders(&self, game_root: &Path) -> Option<Vec<String>> {
        let names = get_all_folder_names(game_root);
        for name in names {
            if game_root.join(&name).join("Binaries").is_dir() {
                return Some(vec![format!("$UNREAL_CONFIG/{}/Saved/SaveGames", name)]);
            }
        }

        None
    }
}
