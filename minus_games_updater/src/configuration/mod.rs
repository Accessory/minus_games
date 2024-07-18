use clap::{command, Parser};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use strum::EnumString;

#[derive(
    Parser,
    Debug,
    Serialize,
    Deserialize,
    strum::Display,
    EnumString,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Clone,
    Copy,
)]
pub enum OS {
    Windows,
    Linux,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct Configuration {
    #[arg(long, env)]
    pub server_url: String,
    #[arg(long, env)]
    pub to: Option<PathBuf>,
    #[cfg(target_family = "windows")]
    #[arg(long, default_value = "Windows", env)]
    pub for_os: OS,
    #[cfg(not(target_family = "windows"))]
    #[arg(long, default_value = "Linux", env)]
    pub for_os: OS,
    #[arg(long, env = "MINUS_GAMES_USERNAME")]
    pub username: Option<String>,
    #[arg(long, env = "MINUS_GAMES_PASSWORD")]
    pub password: Option<String>,
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Server Url: {}", self.server_url.as_str())?;
        writeln!(f, "Update To: {}", self.get_client_to().display())?;
        writeln!(f, "OS: {}", self.for_os)
    }
}

impl Configuration {
    pub fn get_client_to(&self) -> PathBuf {
        match self.to.as_ref() {
            Some(to) => to.clone(),
            None => match self.for_os {
                OS::Windows => PathBuf::from("minus_games_client.exe"),
                OS::Linux => PathBuf::from("minus_games_client"),
            },
        }
    }
}
