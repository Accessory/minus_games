use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::MARGIN_DEFAULT;
use crate::minus_games_gui::views::components_helper::create_info_part;
use crate::minus_games_gui::views::game_helper::create_info_game_line;
use iced::Center;
use iced::widget::{Row, column, horizontal_space, row, text, vertical_space};

pub(crate) fn view(minus_games_gui: &MinusGamesGui) -> Row<MinusGamesGuiMessage> {
    if let Some(cgi) = minus_games_gui.current_game.as_ref() {
        row![
            horizontal_space().width(MARGIN_DEFAULT),
            column![
                create_info_part(
                    cgi,
                    minus_games_gui,
                    "Close Game",
                    MinusGamesGuiMessage::KillCurrentGame
                ),
                create_info_game_line(cgi),
            ]
            .align_x(Center),
            horizontal_space().width(MARGIN_DEFAULT)
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
