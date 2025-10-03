use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{MARGIN_DEFAULT, TEXT};
use iced::widget::space::{horizontal, vertical};
use iced::widget::{Row, column, row, text};

pub(crate) fn view(height: f32) -> Row<'static, MinusGamesGuiMessage> {
    let margin_from_top = (height / 4.0).max(MARGIN_DEFAULT as f32);
    row![
        horizontal(),
        column![
            vertical().height(margin_from_top),
            text("Loading").size(TEXT * 3)
        ],
        horizontal()
    ]
}
