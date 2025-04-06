use crate::constants::{ADDITIONS, HEADER_JPG, INFOS};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use clap::builder::OsStr;
use filetime::set_file_mtime;
use rand_core::OsRng;
use std::hash::{DefaultHasher, Hasher};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::SystemTime;
use walkdir::WalkDir;

pub mod constants;

pub struct CacheFolder {}

static CACHE_FOLDER_PATH: OnceLock<PathBuf> = OnceLock::new();

impl From<CacheFolder> for OsStr {
    fn from(_value: CacheFolder) -> Self {
        CACHE_FOLDER_PATH
            .get_or_init(|| match home::home_dir() {
                None => std::env::current_dir().unwrap().join("cache"),
                Some(value) => value.join(".config").join("minus_games").join("cache"),
            })
            .as_os_str()
            .into()
    }
}
pub struct DataFolder {}

static DATA_FOLDER_PATH: OnceLock<PathBuf> = OnceLock::new();

impl From<DataFolder> for OsStr {
    #[cfg(not(debug_assertions))]
    fn from(_value: DataFolder) -> Self {
        DATA_FOLDER_PATH
            .get_or_init(|| match dirs::data_dir() {
                None => std::env::current_dir().unwrap().join("data"),
                Some(value) => value.join("minus_games").join("data"),
            })
            .as_os_str()
            .into()
    }

    #[cfg(debug_assertions)]
    fn from(_value: DataFolder) -> Self {
        DATA_FOLDER_PATH
            .get_or_init(|| std::env::current_dir().unwrap().join("data"))
            .as_os_str()
            .into()
    }
}
pub struct GamesFolder {}

static GAMES_FOLDER_PATH: OnceLock<PathBuf> = OnceLock::new();

impl From<GamesFolder> for OsStr {
    #[cfg(not(debug_assertions))]
    fn from(_value: GamesFolder) -> Self {
        GAMES_FOLDER_PATH
            .get_or_init(|| match dirs::data_dir() {
                None => std::env::current_dir().unwrap().join("games"),
                Some(value) => value.join("minus_games").join("games"),
            })
            .as_os_str()
            .into()
    }

    #[cfg(debug_assertions)]
    fn from(_value: GamesFolder) -> Self {
        GAMES_FOLDER_PATH
            .get_or_init(|| std::env::current_dir().unwrap().join("games"))
            .as_os_str()
            .into()
    }
}

pub struct ClientGamesFolder {}

static CLIENT_GAMES_FOLDER_PATH: OnceLock<PathBuf> = OnceLock::new();

impl From<ClientGamesFolder> for OsStr {
    #[cfg(not(debug_assertions))]
    fn from(_value: ClientGamesFolder) -> Self {
        CLIENT_GAMES_FOLDER_PATH
            .get_or_init(|| match dirs::data_dir() {
                None => std::env::current_dir().unwrap().join("client_games"),
                Some(value) => value.join("minus_games").join("client_games"),
            })
            .as_os_str()
            .into()
    }

    #[cfg(debug_assertions)]
    fn from(_value: ClientGamesFolder) -> Self {
        CLIENT_GAMES_FOLDER_PATH
            .get_or_init(|| std::env::current_dir().unwrap().join("client_games"))
            .as_os_str()
            .into()
    }
}

pub struct ClientFolder {}

static CLIENT_FOLDER_PATH: OnceLock<PathBuf> = OnceLock::new();

impl From<ClientFolder> for OsStr {
    #[cfg(debug_assertions)]
    fn from(_: ClientFolder) -> Self {
        CLIENT_FOLDER_PATH
            .get_or_init(|| std::env::current_dir().unwrap().join("client"))
            .as_os_str()
            .into()
    }

    #[cfg(not(debug_assertions))]
    fn from(_: ClientFolder) -> Self {
        CLIENT_FOLDER_PATH
            .get_or_init(|| match dirs::config_dir() {
                None => std::env::current_dir().unwrap().join("client_folder"),
                Some(value) => value.join("minus_games").join("client_folder"),
            })
            .as_os_str()
            .into()
    }
}

pub fn create_hash_from_string(value: &str) -> String {
    const CHARS: [char; 62] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut hasher = DefaultHasher::new();
    hasher.write(value.as_bytes());
    let mut number = hasher.finish();
    let mut rtn = String::with_capacity(6);
    while number != 0 {
        let idx = (number % 62) as usize;
        rtn.push(CHARS[idx]);
        if number < 62 {
            break;
        }
        number /= 62;
    }

    rtn
}

pub fn create_file_list(folder: &Path) -> Vec<PathBuf> {
    let mut rtn = Vec::new();
    let iterator = WalkDir::new(folder);

    for dir_entry in iterator.into_iter().flatten() {
        if dir_entry.path().is_file() {
            {
                let file_path = std::path::absolute(dir_entry.path()).unwrap();
                // trace!("Filepath: {}", file_path.display());
                rtn.push(file_path);
            }
        }
    }

    rtn
}

pub fn set_file_modified_time(to: &Path, time: SystemTime) {
    set_file_mtime(to, time.into()).ok();
}

pub fn create_argon2_hash(text: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(text.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_argon2_hash(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash).ok() {
        None => {
            return false;
        }
        Some(parsed_hash) => parsed_hash,
    };
    let argon2 = Argon2::default();
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn create_game_infos_name(game: &str) -> String {
    format!("{game}.json")
}

pub fn get_csv_name(game: &str) -> String {
    format!("{game}.csv")
}

pub fn get_dirty_name(game: &str) -> String {
    format!("{game}.dirty")
}

pub fn create_last_time_played_name(game: &str) -> String {
    format!("{game}.played")
}

pub fn get_game_infos_path(data_dir: &Path, game: &str) -> PathBuf {
    data_dir.join(INFOS).join(create_game_infos_name(game))
}

pub fn get_csv_path(data_dir: &Path, game: &str) -> PathBuf {
    data_dir.join(INFOS).join(get_csv_name(game))
}

pub fn get_dirty_path(data_dir: &Path, game: &str) -> PathBuf {
    data_dir.join(INFOS).join(get_dirty_name(game))
}

pub fn get_last_time_played_path(data_dir: &Path, game: &str) -> PathBuf {
    data_dir
        .join(INFOS)
        .join(create_last_time_played_name(game))
}

pub fn get_header_path(data_dir: PathBuf, game: &str) -> PathBuf {
    data_dir.join(ADDITIONS).join(game).join(HEADER_JPG)
}

#[cfg(test)]
mod tests {
    use crate::create_hash_from_string;
    use crate::{create_argon2_hash, verify_argon2_hash};

    #[test]
    fn test_create_hash_from_string() {
        let value = "test";
        let hash = create_hash_from_string(value);
        assert_eq!(hash, "tMSBYrhFthj");
    }

    #[test]
    fn test_create_argon2_hash() {
        let value = "default";
        let hash = create_argon2_hash(value);
        let verification_true = verify_argon2_hash(value, &hash);
        let verification_false = verify_argon2_hash("value", &hash);
        assert_ne!(verification_false, verification_true);
    }
}
