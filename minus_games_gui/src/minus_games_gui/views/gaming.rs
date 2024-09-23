use crate::minus_games_gui::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::MARGIN_DEFAULT;
use crate::minus_games_gui::MinusGamesGui;
use iced::widget::{column, horizontal_space, row, text, vertical_space, Row};
use iced::Center;

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
