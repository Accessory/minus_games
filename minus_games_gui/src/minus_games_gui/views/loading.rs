use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{MARGIN_DEFAULT, TEXT};
use iced::widget::{Row, column, horizontal_space, row, text, vertical_space};

pub(crate) fn view(height: f32) -> Row<'static, MinusGamesGuiMessage> {
    let margin_from_top = (height / 4.0).max(MARGIN_DEFAULT as f32);
    row![
        horizontal_space(),
        column![
            vertical_space().height(margin_from_top),
            text("Loading").size(TEXT * 3)
        ],
        horizontal_space()
    ]
}
