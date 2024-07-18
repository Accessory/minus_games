use crate::engine_info_functions::EngineInfoFunctions;
use crate::utils::{find_closest_string, find_name_in_folder_name, get_game_exe_or_exe};
use convert_case::{Case, Casing};
use minus_games_models::rpgm_package::RPGMPackage;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct RPGMMZFunctions {}

impl EngineInfoFunctions for RPGMMZFunctions {
    fn get_game_name(&self, game_root: &Path) -> Option<String> {
        let package_path = game_root.join("package.json");
        if package_path.is_file() {
            let mut reader = BufReader::new(File::open(package_path.as_path()).unwrap());
            if let Ok(rpgm_package) = serde_json::from_reader::<_, RPGMPackage>(&mut reader) {
                let mut name = rpgm_package.name;
                name = name.to_case(Case::Title);
                return Some(name);
            }
            let folder_name = game_root.iter().last().unwrap().to_str().unwrap();
            return Some(find_name_in_folder_name(folder_name));
        }
        None
    }

    fn get_linux_exe(&self, game_root: &Path) -> Option<String> {
        if game_root.join("Game").is_file() {
            return Some("Game".into());
        }

        let name = self.get_game_name(game_root)?;
        let mut files: Vec<String> = Vec::new();
        for file_result in game_root.read_dir().unwrap() {
            let file = file_result.unwrap();
            let file_path = file.path();
            if file_path.is_file()
                && file_path.extension().is_none()
                && file.file_name() != "chrome_crashpad_handler"
                && file.file_name() != "crashpad_handler"
                && file.file_name() != "CREDITS"
            {
                files.push(file_path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }
        if files.is_empty() {
            None
        } else {
            let idx = find_closest_string(name.as_str(), &files);
            Some(files.remove(idx))
        }
    }

    fn get_windows_exe(&self, game_root: &Path) -> Option<String> {
        get_game_exe_or_exe(game_root)
    }

    fn get_sync_folders(&self, _: &Path) -> Option<Vec<String>> {
        Some(vec!["$GAME_ROOT/save".to_string()])
    }

    fn get_excludes(&self, _: &Path) -> Option<Vec<String>> {
        Some(vec!["config.rmmzsave".into()])
    }
}
