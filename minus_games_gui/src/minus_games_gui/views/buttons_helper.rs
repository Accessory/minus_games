use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{GAME_CARD_ROW_HEIGHT, TEXT, TOP_BUTTON};
use iced::Fill;
use iced::widget::{Button, button, text};

pub(crate) fn create_config_button<'a>(
    value: &'static str,
    message: MinusGamesGuiMessage,
) -> Button<'a, MinusGamesGuiMessage> {
    button(text(value).size(TEXT).center().width(Fill)) // Quit/Off
        .width(GAME_CARD_ROW_HEIGHT * 2)
        .on_press(message)
        .padding(TOP_BUTTON as u16)
}

pub(crate) fn create_quit_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_config_button("", MinusGamesGuiMessage::CloseApplication(()))
}
