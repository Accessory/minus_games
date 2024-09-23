use crate::minus_games_gui::minus_games_gui_message::MinusGamesGuiMessage;
use iced::widget::{horizontal_space, row, text, Row};

pub(crate) fn view() -> Row<'static, MinusGamesGuiMessage> {
    row![
        horizontal_space(),
        text("Loading").size(50),
        horizontal_space()
    ]
}
