use std::path::Path;

use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::find_all_possible_game_exe_files;

#[derive(Copy, Clone)]
pub struct FinalFantasyPixelRemasterFunctions {}

impl EngineInfoFunctions for FinalFantasyPixelRemasterFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        let exe_name = self.get_windows_exe(game_root)?;
        let split = exe_name[0..exe_name.len() - 4].split(' ');
        let name = format!("Final Fantasy {} Pixel Remaster", split.last()?);
        Some(name)
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        let files: Vec<String> = find_all_possible_game_exe_files(game_root);
        files
            .into_iter()
            .find(|file| file.to_lowercase().contains("final fantasy"))
    }

    fn get_sync_folders(&self, game_root: &Path) -> Option<Vec<String>> {
        let exe_name = self.get_windows_exe(game_root)?;
        let split = exe_name[0..exe_name.len() - 4].split(' ');
        let save_folder = format!("$DOCUMENTS/My Games/FINAL FANTASY {} PR", split.last()?);
        Some(vec![save_folder])
    }
}
