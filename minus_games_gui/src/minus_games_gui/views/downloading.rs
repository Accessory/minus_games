use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::MARGIN_DEFAULT;
use crate::minus_games_gui::views::components_helper::create_info_part;
use crate::minus_games_gui::views::game_helper::create_info_game_line;
use iced::widget::{Row, column, horizontal_space, progress_bar, row, stack, text, vertical_space};
use iced::{Center, Fill};

pub(crate) fn view(minus_games_gui: &MinusGamesGui) -> Row<'_, MinusGamesGuiMessage> {
    if let Some(cgi) = minus_games_gui.current_game.as_ref() {
        row![
            horizontal_space().width(MARGIN_DEFAULT),
            column![
                create_info_part(
                    cgi,
                    minus_games_gui,
                    "Stop Download",
                    MinusGamesGuiMessage::StopDownload
                ),
                stack!(
                    progress_bar(
                        0.0..=minus_games_gui.files_to_download as f32,
                        minus_games_gui.files_downloaded as f32
                    )
                    .length(Fill),
                    text(format!(
                        "{}/{}",
                        minus_games_gui.files_downloaded, minus_games_gui.files_to_download
                    ))
                    .width(Fill)
                    .height(Fill)
                    .center()
                ),
                vertical_space().height(MARGIN_DEFAULT),
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
                text("Downloading").size(50),
            ],
            horizontal_space(),
        ]
    }
}
