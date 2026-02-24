use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{GAME_CARD_ROW_HEIGHT, TOP_BUTTON_HEIGHT};
use crate::minus_games_gui::views::icons::{
    ARROW_LEFT, ARROW_ROTATE_RIGHT, FLOPPY_DISK, ON_OFF, SLIDERS,
};
use iced::widget::svg::Status;
use iced::widget::{Button, button, svg};
use iced::{Fill, Theme};

pub(crate) fn create_svg_config_button<'a>(
    value: svg::Handle,
    message: MinusGamesGuiMessage,
) -> Button<'a, MinusGamesGuiMessage> {
    button(svg(value).style(set_svg_style).width(Fill).height(Fill))
        .width(GAME_CARD_ROW_HEIGHT)
        .height(TOP_BUTTON_HEIGHT)
        .on_press(message)
    // .padding(TOP_BUTTON as u16)
}

pub(crate) fn create_quit_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_svg_config_button(ON_OFF.clone(), MinusGamesGuiMessage::CloseApplication(()))
}

pub(crate) fn create_save_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_svg_config_button(
        FLOPPY_DISK.clone(),
        MinusGamesGuiMessage::BackFromSettings(true),
    )
}

pub(crate) fn create_back_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_svg_config_button(
        ARROW_LEFT.clone(),
        MinusGamesGuiMessage::BackFromSettings(false),
    )
}

pub(crate) fn create_reload_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_svg_config_button(ARROW_ROTATE_RIGHT.clone(), MinusGamesGuiMessage::Reload)
}
pub(crate) fn create_settings_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_svg_config_button(SLIDERS.clone(), MinusGamesGuiMessage::GotoSettings)
}

pub(crate) fn set_svg_style(theme: &Theme, _status: Status) -> svg::Style {
    let palette = theme.extended_palette();
    svg::Style {
        color: Some(palette.background.base.text),
    }
}

// pub(crate) fn create_config_button<'a>(
//     value: &'static str,
//     message: MinusGamesGuiMessage,
// ) -> Button<'a, MinusGamesGuiMessage> {
//     button(
//         text(value)
//             .size(TEXT)
//             .center()
//             .width(Fill)
//             .font(DEFAULT_FONT),
//     ) // Quit/Off
//     .width(GAME_CARD_ROW_HEIGHT)
//     .on_press(message)
//     // .padding(TOP_BUTTON as u16)
// }
