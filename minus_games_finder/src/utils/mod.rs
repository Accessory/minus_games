use crate::configuration::Configuration;
use convert_case::{Case, Casing};
use glob::MatchOptions;
use minus_games_models::game_file_info::GameFileInfo;
use minus_games_models::game_infos::GameInfos;
use minus_games_utils::create_file_list;
use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use textdistance::str::damerau_levenshtein;
use tracing::{trace, warn};
use walkdir::WalkDir;

pub(crate) fn is_elf(path: &Path) -> bool {
    // First 7 Bytes of a ELF 64 executable
    const ELF_64: [u8; 7] = [0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01];
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return false;
        }
    };
    let mut buffer = [0; 7];
    match file.read_exact(&mut buffer) {
        Ok(_) => {}
        Err(_) => {
            return false;
        }
    };
    buffer == ELF_64
}

pub fn find_name_in_folder_name(folder_name: &str) -> String {
    let mut end = 0;

    for (pos, _) in folder_name.as_bytes().iter().enumerate().skip(1) {
        let slice = &folder_name.as_bytes()[pos..];

        if slice.starts_with(b" v") && slice.get(2).is_some_and(|i| i.is_ascii_digit()) {
            break;
        }

        if slice.starts_with(b"-v") && slice.get(2).is_some_and(|i| i.is_ascii_digit()) {
            break;
        }
        end = pos + 1;
    }

    String::from_str(&folder_name[0..end]).unwrap()
}

pub fn find_closest_string(close_to: &str, list: &[String]) -> usize {
    let mut min = usize::MAX;
    let mut rtn = 0;

    for (idx, item) in list.iter().enumerate() {
        let distance = damerau_levenshtein(close_to, item);
        if distance < min {
            rtn = idx;
            min = distance;
        }
    }

    rtn
}

pub fn save_game_file_infos(game_folder: &Path, config: &Configuration, game_infos: &GameInfos) {
    let file_list = create_file_list(game_folder);

    let csv_name = format!("{}.csv", game_infos.folder_name.as_str());
    let csv_path = config.data_folder.join(csv_name);
    if let Err(err) = std::fs::create_dir_all(config.data_folder.as_path()) {
        warn!("Failed to create client data folder: {}", err);
        return;
    };
    let mut csv_writer = csv::Writer::from_path(csv_path.as_path()).unwrap();
    let cut_off = std::path::absolute(config.games_folder.as_path())
        .unwrap()
        .iter()
        .count();

    for file in file_list {
        let file_info: GameFileInfo = GameFileInfo::from_path_buf_with_cut_off(file, cut_off);
        trace!("Game File Info: {file_info}");
        csv_writer.serialize(file_info).unwrap()
    }
    csv_writer.flush().unwrap();
}

pub fn save_infos_to_data_folder(data_folder: &Path, game_infos: &GameInfos) {
    if !data_folder.is_dir() {
        std::fs::create_dir_all(data_folder).expect("Failed to create data folder");
    }

    let json_name = format!("{}.json", game_infos.folder_name.as_str());
    let json_path = data_folder.join(json_name);

    std::fs::write(json_path, game_infos.to_string()).expect("Unable to write game infos to file");
}

pub fn file_path_is_windows_exe(file_path: &Path) -> bool {
    file_path.is_file() && file_path.extension().is_some_and(|e| e == "exe")
}

pub fn find_all_possible_game_exe_files(game_folder: &Path) -> Vec<String> {
    let mut files = Vec::new();

    for file_result in game_folder.read_dir().unwrap() {
        let file = file_result.unwrap();
        let file_path = file.path();
        if file_path_is_windows_exe(&file_path) {
            let file_name = file.file_name().to_str().unwrap().to_lowercase();

            if !["uninstall.exe", "unins000.exe"].contains(&file_name.as_str()) {
                files.push(file_path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }
    }

    files
}

pub fn get_title_from_parent_folder(root: &Path) -> Option<String> {
    let folder_name = root.iter().next_back().unwrap().to_str().unwrap();
    Some(find_name_in_folder_name(
        folder_name.to_case(Case::Title).as_str(),
    ))
}

pub fn get_all_folder_names(root: &Path) -> Vec<String> {
    let dir = std::fs::read_dir(root).unwrap();
    let mut rtn_folders = Vec::new();
    for entry in dir.flatten() {
        if entry.path().is_dir() {
            rtn_folders.push(
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
            );
        }
    }
    rtn_folders
}

pub fn find_possible_save_dir_in_game_root(game_root: &Path) -> Option<Vec<String>> {
    let walker = WalkDir::new(game_root).into_iter();
    for entry in walker.filter_entry(|e| e.path().is_dir()) {
        let e = entry.ok()?;
        let name = e.file_name().to_ascii_lowercase();
        if name == "save" || name == "saves" || name == "savedata" {
            let cut_off = game_root.iter().count();
            let path_parts: Vec<&OsStr> = e.path().iter().skip(cut_off).collect();
            let path = path_parts.join(OsStr::new("/"));
            let path = format!("$GAME_ROOT/{}", path.into_string().ok()?);
            return Some(vec![path]);
        }
    }
    None
}

pub fn get_closest_windows_exe(name: &str, folder: &Path) -> Option<String> {
    let mut files: Vec<String> = find_all_possible_game_exe_files(folder);
    if files.is_empty() {
        None
    } else {
        let idx = find_closest_string(name, &files);
        Some(files.remove(idx))
    }
}

pub fn return_closed_string(name: &str, mut files: Vec<String>) -> Option<String> {
    match files.len() {
        0 => None,
        1 => Some(files.remove(0)),
        _ => {
            let idx = find_closest_string(name, &files);
            Some(files.remove(idx))
        }
    }
}

pub fn glob_file_path(root: &Path, to_join: &str) -> Option<PathBuf> {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    let glob_str = root.join(to_join).to_str().unwrap().to_string();
    let mut results = glob::glob_with(&glob_str, options).unwrap();
    if let Some(Ok(first)) = results.next() {
        Some(first)
    } else {
        None
    }
}

pub fn get_game_exe_or_exe(folder: &Path) -> Option<String> {
    const GAME_EXE: &str = "Game.exe";
    let game_exe = folder.join(GAME_EXE);
    if game_exe.is_file() {
        return Some(GAME_EXE.into());
    }
    glob_for_file(folder, "*.exe")
}

pub fn get_closest_exe_from_folder(folder: &Path, name: &str) -> Option<String> {
    let mut files: Vec<String> = find_all_possible_game_exe_files(folder);
    if files.is_empty() {
        None
    } else {
        let idx = find_closest_string(name, &files);
        Some(files.remove(idx))
    }
}

pub fn glob_for_file(root: &Path, to_join: &str) -> Option<String> {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    let glob_str = root.join(to_join).to_str().unwrap().to_string();
    let mut results = glob::glob_with(&glob_str, options).unwrap();

    if let Some(first) = results.next() {
        let res = first.unwrap();
        return Some(res.file_name().unwrap().to_str().unwrap().to_string());
    }
    None
}
