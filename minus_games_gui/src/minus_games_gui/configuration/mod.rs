use clap::{Parser, command};
use iced::Font;
use std::fmt::{Display, Formatter};

pub(crate) const DEFAULT_THEME_NAME: &str = "Light";
pub(crate) const DEFAULT_FONT_NAME: &str = "MonaspiceAr Nerd Font";
pub(crate) static DEFAULT_FONT: Font = Font::with_name(DEFAULT_FONT_NAME);

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
    #[arg(long, env = "MINUS_GAMES_GUI_THEME", default_value = DEFAULT_THEME_NAME)]
    pub theme: String,
    #[arg(long, env = "MINUS_GAMES_GUI_SCALE")]
    pub scale: Option<f64>,
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
        writeln!(f, "Theme: {}", &self.theme)?;
        write!(f, "Font: {}", &self.font)
    }
}
