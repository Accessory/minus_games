use crate::minus_games_gui::configuration::{DEFAULT_FONT_NAME, Mode};
use clap::Parser;
use std::fmt::{Display, Formatter};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct GuiConfiguration {
    #[arg(long, env = "MINUS_GAMES_GUI_FULLSCREEN")]
    pub fullscreen: bool,
    #[arg(long, env = "MINUS_GAMES_GUI_MODE", default_value = "Gui")]
    pub mode: Mode,
    #[arg(long, env = "MINUS_GAMES_GUI_THEME")]
    pub theme: Option<String>,
    #[arg(long, env = "MINUS_GAMES_GUI_SCALE")]
    pub scale: Option<f32>,
    #[arg(
        long,
        env = "MINUS_GAMES_GUI_FONT",
        default_value = DEFAULT_FONT_NAME
    )]
    pub font: String,
}

impl Display for GuiConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Fullscreen: {}", self.fullscreen)?;
        writeln!(f, "Mode: {}", self.mode)?;
        writeln!(
            f,
            "Theme: {}",
            self.theme.as_ref().unwrap_or(&String::from("System"))
        )?;
        write!(f, "Font: {}", self.font)
    }
}
