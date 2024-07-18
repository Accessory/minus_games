use std::fs;
use std::path::Path;

use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{
    find_possible_save_dir_in_game_root, get_closest_exe_from_folder, glob_file_path, glob_for_file,
};

#[derive(Copy, Clone)]
pub struct UnityFunctions {}

impl EngineInfoFunctions for UnityFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        if let Some(file) = glob_file_path(game_root, "*/app.info") {
            let content = fs::read_to_string(file).ok()?;
            let name = content.lines().nth(1)?.to_string();
            Some(name)
        } else {
            None
        }
    }

    fn get_linux_exe(&self, game_root: &Path) -> Option<String> {
        glob_for_file(game_root, "*.x86_64")
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        let name = self.get_game_name(game_root)?;
        get_closest_exe_from_folder(game_root, name.as_str())
    }

    fn get_sync_folders(&self, game_root: &Path) -> Option<Vec<String>> {
        if let Some(file) = glob_file_path(game_root, "*/app.info") {
            let content = fs::read_to_string(file).ok()?;
            let mut split = content.split('\n');
            let mut rtn = vec![format!("$UNITY_CONFIG/{}/{}", split.next()?, split.next()?)];

            if let Some(mut save_folder) = find_possible_save_dir_in_game_root(game_root) {
                rtn.append(&mut save_folder);
            }

            Some(rtn)
        } else {
            None
        }
    }

    fn get_excludes(&self, _game_root: &Path) -> Option<Vec<String>> {
        Some(vec![
            "Player.log".into(),
            "Player-prev.log".into(),
            "output_log.txt".into(),
            "prefs".into(),
            "Analytics".into(),
        ])
    }
}
