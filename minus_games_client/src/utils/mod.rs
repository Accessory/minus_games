#[cfg(target_family = "unix")]
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt::Display;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
#[cfg(target_family = "unix")]
use tracing::debug;

pub fn get_json_name(game: &str) -> String {
    format!("{game}.json")
}

pub fn get_csv_name(game: &str) -> String {
    format!("{game}.csv")
}

#[allow(dead_code)]
#[cfg(target_family = "unix")]
pub fn is_executable(mode: u32) -> bool {
    mode & 0o111 != 0
}

#[cfg(target_family = "unix")]
pub fn is_not_executable(mode: u32) -> bool {
    mode & 0o111 == 0
}

#[cfg(target_family = "unix")]
pub fn make_executable_from_path(path: &Path) {
    let mode = path.metadata().unwrap().permissions().mode();
    make_executable(path, mode);
}

#[cfg(target_family = "unix")]
pub fn make_executable(path: &Path, mut mode: u32) {
    debug!("Make file {}, executable", path.display());
    mode |= 0o111;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
}

pub fn is_or_none_path_buf(object: &Option<PathBuf>) -> &str {
    match object.as_ref() {
        None => "None",
        Some(value) => value.as_os_str().to_str().unwrap(),
    }
}
pub fn is_or_none_string(object: &Option<String>) -> &str {
    match object.as_ref() {
        None => "None",
        Some(value) => value.as_str(),
    }
}

pub fn is_or_none<T>(object: Option<&T>) -> String
where
    T: Display,
{
    match object {
        None => "None".into(),
        Some(content) => {
            format!("{content}")
        }
    }
}

#[cfg(target_family = "unix")]
pub fn add_permissions(game_path: &Path, exe_stem: &OsStr) {
    for entry in walkdir::WalkDir::new(game_path).into_iter().flatten() {
        let path = entry.path();
        if path.is_file()
            && (path.extension().is_none() || path.file_name().unwrap() == exe_stem)
            && is_not_filtered(path)
        {
            make_executable_from_path(path);
        }
    }
}

#[cfg(target_family = "unix")]
fn is_not_filtered(path: &Path) -> bool {
    const FILTER_LIST: [&str; 4] = ["save", "assets", "monobleedingedge", "resources"];
    let lower_path = path.to_str().unwrap().to_lowercase();
    for filter_item in FILTER_LIST {
        if lower_path.contains(filter_item) {
            return false;
        }
    }

    true
}

pub fn encode_problem_chars(text: &str) -> String {
    text.replace('?', "%3F").replace(':', "%3A")
}

pub fn get_folders_in_path(path: &Path) -> Vec<OsString> {
    path.read_dir()
        .expect("Failed to read game folder")
        .map(|rd| rd.unwrap().path())
        .filter(|i| i.is_dir())
        .map(|i| i.file_name().unwrap().to_os_string())
        .collect()
}
