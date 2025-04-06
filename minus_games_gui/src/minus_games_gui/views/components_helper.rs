use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{
    BIG_IMAGE_HEIGHT, LONG_BUTTON_WIDTH, MARGIN_DEFAULT, TEXT,
};
use iced::widget::text::Wrapping;
use iced::widget::{Column, button, horizontal_space, row, text, vertical_space};
use iced::{Center, Fill, Left};
use minus_games_models::game_infos::GameInfos;

pub fn create_info_part<'a>(
    game_infos: &'a GameInfos,
    minus_games_gui: &'a MinusGamesGui,
    button_text: &'a str,
    button_message: MinusGamesGuiMessage,
) -> Column<'a, MinusGamesGuiMessage> {
    let current_game_card = minus_games_gui
        .game_cards
        .iter()
        .find(|gc| gc.game == game_infos.folder_name)
        .unwrap();

    let mut rtn = create_top_part(&game_infos.folder_name, button_text, button_message);
    if let Some(handle) = current_game_card.image.as_ref() {
        rtn = rtn.push(row![
            horizontal_space().height(MARGIN_DEFAULT),
            iced::widget::image(handle.clone()).height(BIG_IMAGE_HEIGHT),
            horizontal_space().height(MARGIN_DEFAULT)
        ]);
    }
    rtn.width(Fill).padding(MARGIN_DEFAULT as u16)
}

fn create_top_part<'a>(
    game_name: &'a str,
    button_text: &'a str,
    button_message: MinusGamesGuiMessage,
) -> Column<'a, MinusGamesGuiMessage> {
    iced::widget::column![
        row![
            text("Downloading:").size(TEXT).align_x(Left).width(Fill),
            horizontal_space().width(MARGIN_DEFAULT),
            button(text(button_text).width(Fill).align_x(Center))
                .width(LONG_BUTTON_WIDTH)
                .on_press(button_message)
        ],
        text(game_name)
            .size(TEXT)
            .align_x(Left)
            .width(Fill)
            .wrapping(Wrapping::None)
            .height(TEXT),
        vertical_space().height(MARGIN_DEFAULT),
    ]
    .align_x(Center)
}
