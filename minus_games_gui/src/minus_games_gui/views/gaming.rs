use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{
    BIG_IMAGE_HEIGHT, LONG_BUTTON_WIDTH, MARGIN_DEFAULT, TEXT,
};
use iced::Length::Fill;
use iced::widget::{Column, Row, button, column, horizontal_space, row, text, vertical_space};
use iced::{Center, Left};
use minus_games_models::game_infos::GameInfos;

pub(crate) fn view(minus_games_gui: &MinusGamesGui) -> Row<MinusGamesGuiMessage> {
    if let Some(cgi) = minus_games_gui.current_game.as_ref() {
        row![
            horizontal_space().width(MARGIN_DEFAULT),
            column![
                create_top(cgi, minus_games_gui).width(Fill),
                text(format!("Engine: {}", cgi.engine)),
                text(format!("Linux support: {}", cgi.supported_platforms.linux)),
                text(format!(
                    "Windows support: {}",
                    cgi.supported_platforms.windows
                )),
                vertical_space().height(MARGIN_DEFAULT)
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

fn create_top<'a>(
    game_infos: &'a GameInfos,
    minus_games_gui: &'a MinusGamesGui,
) -> Column<'a, MinusGamesGuiMessage> {
    let current_game_card = minus_games_gui
        .game_cards
        .iter()
        .find(|gc| gc.game == game_infos.folder_name)
        .unwrap();

    if let Some(handle) = current_game_card.image.as_ref() {
        column![
            row![
                text("Playing:").size(TEXT).align_x(Left).width(Fill),
                horizontal_space().width(MARGIN_DEFAULT),
                button(text("Stop Game").width(Fill).align_x(Center))
                    .width(LONG_BUTTON_WIDTH)
                    .on_press(MinusGamesGuiMessage::KillCurrentGame)
            ],
            text(&game_infos.folder_name)
                .size(TEXT)
                .align_x(Left)
                .width(Fill),
            vertical_space().height(MARGIN_DEFAULT),
            row![
                horizontal_space().height(MARGIN_DEFAULT),
                iced::widget::image(handle.clone()).height(BIG_IMAGE_HEIGHT),
                horizontal_space().height(MARGIN_DEFAULT)
            ]
        ]
        .align_x(Center)
        .padding(MARGIN_DEFAULT as u16)
    } else {
        column![
            text(format!("Playing: {}", &game_infos.folder_name))
                .size(TEXT)
                .align_x(Center)
                .width(Fill),
            button(text("Stop Game").width(Fill).align_x(Center))
                .width(LONG_BUTTON_WIDTH)
                .on_press(MinusGamesGuiMessage::KillCurrentGame)
        ]
        .align_x(Center)
        .width(Fill)
    }
}
