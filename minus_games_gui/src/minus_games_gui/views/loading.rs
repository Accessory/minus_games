use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use iced::widget::{Row, horizontal_space, row, text};

pub(crate) fn view() -> Row<'static, MinusGamesGuiMessage> {
    row![
        horizontal_space(),
        text("Loading").size(50),
        horizontal_space()
    ]
}
