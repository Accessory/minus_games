use clap::{Parser, command};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum::Display, strum::EnumString, Default)]
pub(crate) enum Mode {
    #[default]
    Gui,
    Cli,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct GuiConfiguration {
    #[arg(long, env = "MINUS_GAMES_GUI_FULLSCREEN")]
    pub fullscreen: bool,
    #[arg(long, env = "MINUS_GAMES_GUI_MODE", default_value = "Gui")]
    pub mode: Mode,
    #[arg(long, env = "MINUS_GAMES_GUI_THEME", default_value = "Light")]
    pub theme: String,
}

impl Display for GuiConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Fullscreen: {}", self.fullscreen)?;
        writeln!(f, "Mode: {}", self.mode)?;
        write!(f, "Theme: {}", &self.theme)
    }
}
