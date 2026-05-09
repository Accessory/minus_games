use iced::Font;

pub(crate) mod complete_configuration;
pub(crate) mod gui_configuration;

// pub(crate) const DEFAULT_THEME_NAME: &str = "Light";
pub(crate) const DEFAULT_FONT_NAME: &str = "MonaspiceAr Nerd Font";
pub(crate) static DEFAULT_FONT: Font = Font::new(DEFAULT_FONT_NAME);

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum::Display, strum::EnumString, Default)]
pub(crate) enum Mode {
    #[default]
    Gui,
    Cli,
}
