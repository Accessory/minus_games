use crate::minus_games_gui::configuration::DEFAULT_FONT;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use iced::widget::{Column, Row, column, text};
use iced::{Center, Fill};
use minus_games_models::game_infos::GameInfos;

pub(crate) fn create_info_game_line(cgi: &GameInfos) -> Column<'_, MinusGamesGuiMessage> {
    create_info_game_line_with(
        cgi.engine.to_string(),
        cgi.supports_linux(),
        cgi.supports_windows(),
    )
}

pub(crate) fn create_info_game_line_with(
    engine: String,
    supports_linux: bool,
    supports_windows: bool,
) -> Column<'static, MinusGamesGuiMessage> {
    let column = column![text(format!("Engine: {engine} "))];
    let mut row = Row::new();
    if supports_linux {
        row = row.push(text(" ").font(DEFAULT_FONT));
    }
    if supports_windows {
        row = row.push(text("").font(DEFAULT_FONT));
    }
    row = row.align_y(Center);
    column.push(row).width(Fill).align_x(Center)
}
