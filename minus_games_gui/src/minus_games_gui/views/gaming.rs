use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{LONG_BUTTON_WIDTH, MARGIN_DEFAULT};
use crate::minus_games_gui::MinusGamesGui;
use iced::widget::{button, column, horizontal_space, row, text, vertical_space, Row};
use iced::Center;
use iced::Length::Fill;

pub(crate) fn view(minus_games_gui: &MinusGamesGui) -> Row<MinusGamesGuiMessage> {
    if let Some(cgi) = minus_games_gui.current_game.as_ref() {
        row![
            horizontal_space(),
            column![
                vertical_space().height(MARGIN_DEFAULT),
                text("Gaming").size(50),
                text(format!("Game: {}", cgi.name)),
                text(format!("Engine: {}", cgi.engine)),
                text(format!("Linux support: {}", cgi.supported_platforms.linux)),
                text(format!(
                    "Windows support: {}",
                    cgi.supported_platforms.windows
                )),
                vertical_space().height(MARGIN_DEFAULT),
                button(text("Stop Game").width(Fill).align_x(Center))
                    .width(LONG_BUTTON_WIDTH)
                    .on_press(MinusGamesGuiMessage::KillCurrentGame)
            ]
            .align_x(Center),
            horizontal_space()
        ]
    } else {
        row![
            horizontal_space(),
            column![
                vertical_space().height(MARGIN_DEFAULT),
                text("Gaming").size(50),
            ],
            horizontal_space(),
        ]
    }
}
