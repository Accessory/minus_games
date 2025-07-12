use std::path::Path;

use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{
    find_closest_string, get_all_folder_names, get_closest_exe_from_folder,
    get_title_from_parent_folder,
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
        let mut possible_folders = get_all_folder_names(game_root);

        possible_folders.retain(|name| game_root.join(name).join("Binaries").is_dir());

        if possible_folders.is_empty() {
            return None;
        }

        let game_binary_name = if possible_folders.len() == 1 {
            possible_folders.first()?
        } else {
            let windows_exe = self.get_windows_exe(game_root)?;
            possible_folders.retain(|e| e != "Engine");
            let result =
                find_closest_string(&windows_exe[0..windows_exe.len() - 4], &possible_folders);
            possible_folders.get(result)?
        };

        Some(vec![format!(
            "$UNREAL_CONFIG/{}/Saved/SaveGames",
            game_binary_name
        )])
    }
}
