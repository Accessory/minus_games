use clap::{Parser, command};
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
        writeln!(f, "Update Client To: {}", self.get_client_to().display())?;
        writeln!(f, "Update Gui To: {}", self.get_gui_to().display())?;
        writeln!(f, "OS: {}", self.for_os)
    }
}

impl Configuration {
    pub fn get_client_to(&self) -> PathBuf {
        let name = match self.for_os {
            OS::Windows => "minus_games_client.exe",
            OS::Linux => "minus_games_client",
        };
        match self.to.as_ref() {
            Some(to) => to.join(name),
            None => PathBuf::from(name),
        }
    }
    pub fn get_gui_to(&self) -> PathBuf {
        let name = match self.for_os {
            OS::Windows => "minus_games_gui.exe",
            OS::Linux => "minus_games_gui",
        };
        match self.to.as_ref() {
            Some(to) => to.join(name),
            None => PathBuf::from(name),
        }
    }
}
