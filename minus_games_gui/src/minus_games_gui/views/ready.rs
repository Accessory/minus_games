use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::MARGIN_DEFAULT;
use iced::widget::{Row, row};

pub(crate) fn view(minus_games_gui: &MinusGamesGui) -> Row<'_, MinusGamesGuiMessage> {
    row![minus_games_gui.create_ready_view()].padding(MARGIN_DEFAULT as u16)
}
