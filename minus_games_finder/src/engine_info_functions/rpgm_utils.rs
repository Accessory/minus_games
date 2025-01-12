use std::{fs::File, io::BufReader, path::Path};

use convert_case::{Case, Casing};
use minus_games_models::rpgm_package::RPGMPackage;

use crate::utils::{find_closest_string, find_name_in_folder_name};

use super::EngineInfoFunctions;

pub(crate) fn get_rpgm_linux_exe(
    eif: &impl EngineInfoFunctions,
    game_root: &Path,
) -> Option<String> {
    if game_root.join("Game").is_file() {
        return Some("Game".into());
    }

    if game_root.join("Game.sh").is_file() {
        return Some("Game.sh".into());
    }

    let name = eif.get_game_name(game_root)?;
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

pub(crate) fn get_rpgm_name(game_root: &Path) -> Option<String> {
    let package_path = game_root.join("package.json");
    if package_path.is_file() {
        let mut reader = BufReader::new(File::open(package_path.as_path()).unwrap());
        if let Ok(rpgm_package) = serde_json::from_reader::<_, RPGMPackage>(&mut reader) {
            let mut name = rpgm_package.name;
            name = name.to_case(Case::Title);
            return Some(name);
        }
        let folder_name = game_root.iter().next_back().unwrap().to_str().unwrap();
        return Some(find_name_in_folder_name(folder_name));
    }
    None
}
