#![feature(let_chains)]

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

pub struct CacheFolder {}

static CACHE_FOLDER_PATH: OnceLock<PathBuf> = OnceLock::new();

impl From<CacheFolder> for OsStr {
    fn from(_value: CacheFolder) -> Self {
        CACHE_FOLDER_PATH
            .get_or_init(|| match home::home_dir() {
                None => PathBuf::from("./"),
                Some(value) => value.join(".config").join("minus_games").join("cache"),
            })
            .as_os_str()
            .into()
    }
}
pub struct DataFolder {}

static DATA_FOLDER_PATH: OnceLock<PathBuf> = OnceLock::new();

impl From<DataFolder> for OsStr {
    fn from(_value: DataFolder) -> Self {
        DATA_FOLDER_PATH
            .get_or_init(|| match home::home_dir() {
                None => PathBuf::from("./"),
                Some(value) => value.join(".config").join("minus_games").join("data"),
            })
            .as_os_str()
            .into()
    }
}

pub struct ClientFolder {}

static CLIENT_FOLDER_PATH: OnceLock<PathBuf> = OnceLock::new();

impl From<ClientFolder> for OsStr {
    fn from(_: ClientFolder) -> Self {
        CLIENT_FOLDER_PATH
            .get_or_init(|| match home::home_dir() {
                None => PathBuf::from("./"),
                Some(value) => value
                    .join(".config")
                    .join("minus_games")
                    .join("client_folder"),
            })
            .as_os_str()
            .into()
    }
}

static CWD_FOLDER_PATH: OnceLock<PathBuf> = OnceLock::new();

pub struct CurrentDir {}

impl From<CurrentDir> for OsStr {
    fn from(_value: CurrentDir) -> Self {
        CWD_FOLDER_PATH
            .get_or_init(|| {
                std::env::current_dir().expect("Could not get the current working directory")
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

    for item in iterator {
        if let Ok(dir_entry) = item
            && dir_entry.path().is_file()
        {
            let file_path = std::path::absolute(dir_entry.path()).unwrap();
            // trace!("Filepath: {}", file_path.display());
            rtn.push(file_path);
        }
    }

    rtn
}

pub fn set_file_modified_time(to: &Path, time: SystemTime) {
    set_file_mtime(to, time.into()).unwrap();
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
