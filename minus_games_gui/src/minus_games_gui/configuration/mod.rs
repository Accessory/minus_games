use clap::{command, Parser};
use std::fmt::{Display, Formatter};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct GuiConfiguration {
    #[arg(long, env = "MINUS_GAMES_GUI_FULLSCREEN")]
    pub fullscreen: bool,
    #[arg(long, env = "MINUS_GAMES_GUI_CLI")]
    pub cli: bool,
}

pub(crate) const GUI_CONFIGURATION_OPTIONS: [&'static str; 2] = ["--fullscreen", "--cli"];

impl Display for GuiConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Fullscreen: {}", self.fullscreen)?;
        write!(f, "Cli: {}", self.cli)
    }
}
