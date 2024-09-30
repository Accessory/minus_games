use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{
    find_possible_save_dir_in_game_root, get_closest_windows_exe, get_title_from_parent_folder,
    is_elf, return_closed_string,
};
use std::path::Path;

#[derive(Copy, Clone)]
pub struct OtherFunctions {}

impl EngineInfoFunctions for OtherFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        get_title_from_parent_folder(game_root)
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        let name = self.get_game_name(game_root)?;
        if game_root.join("Game.exe").is_file() {
            return Some("Game.exe".into());
        }
        get_closest_windows_exe(name.as_str(), game_root)
    }

    fn get_linux_exe(&self, game_root: &Path) -> Option<String> {
        let mut potentials: Vec<String> = Vec::new();
        for dir_entry in std::fs::read_dir(game_root).unwrap().filter_map(|f| f.ok()) {
            let path = dir_entry.path();

            if !path.is_file() {
                continue;
            }

            let extension_option = path.extension();
            if let Some(extension) = extension_option {
                match extension.to_str().unwrap_or_default() {
                    "x86_64" | "sh" => {
                        potentials.push(dir_entry.path().file_name().unwrap().to_str()?.to_string())
                    }
                    _ => {}
                }
            } else {
                let file_name = path.file_name()?.to_ascii_lowercase();
                if (file_name != "version" || file_name != "readme" || file_name != "notes")
                    && is_elf(&path)
                {
                    potentials.push(dir_entry.path().file_name().unwrap().to_str()?.to_string());
                }
            }
        }
        let name = self.get_game_name(game_root)?;
        return_closed_string(name.as_str(), potentials)
    }

    fn get_sync_folders(&self, game_root: &Path) -> Option<Vec<String>> {
        find_possible_save_dir_in_game_root(game_root)
    }
}
