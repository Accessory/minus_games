use clap::{command, Parser};
use std::fmt::{Display, Formatter};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct GuiConfiguration {
    #[arg(long, env = "MINUS_GAMES_GUI_FULLSCREEN")]
    pub fullscreen: bool,
    #[arg(long, env = "MINUS_GAMES_GUI_CLI")]
    pub cli: bool,
    #[arg(long, env = "MINUS_GAMES_GUI_THEME")]
    pub theme: Option<String>,
}

pub(crate) const GUI_CONFIGURATION_OPTIONS: [&str; 2] = ["--fullscreen", "--cli"];

impl Display for GuiConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Fullscreen: {}", self.fullscreen)?;
        writeln!(f, "Cli: {}", self.cli)?;
        write!(f, "Theme: {}", self.theme.as_deref().unwrap_or("Light"))
    }
}
