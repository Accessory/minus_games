use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{LONG_BUTTON_WIDTH, MARGIN_DEFAULT};
use crate::minus_games_gui::MinusGamesGui;
use iced::widget::{
    button, column, horizontal_space, progress_bar, row, text, vertical_space, Row,
};
use iced::{Center, Fill, Length};

pub(crate) fn view(minus_games_gui: &MinusGamesGui) -> Row<MinusGamesGuiMessage> {
    if let Some(cgi) = minus_games_gui.current_game.as_ref() {
        row![
            horizontal_space().width(MARGIN_DEFAULT),
            column![
                vertical_space().height(MARGIN_DEFAULT),
                text("Downloading").size(50),
                text(format!("Game: {}", cgi.folder_name)),
                text(format!("Engine: {}", cgi.engine)),
                text(format!("Linux support: {}", cgi.supported_platforms.linux)),
                text(format!(
                    "Windows support: {}",
                    cgi.supported_platforms.windows
                )),
                vertical_space().height(MARGIN_DEFAULT),
                progress_bar(
                    0.0..=minus_games_gui.files_to_download as f32,
                    minus_games_gui.files_downloaded as f32
                )
                .width(Length::Fill),
                vertical_space().height(MARGIN_DEFAULT),
                text(format!(
                    "{}/{}",
                    minus_games_gui.files_downloaded, minus_games_gui.files_to_download
                ))
                .size(50),
                button(text("Stop Download").width(Fill).align_x(Center))
                    .width(LONG_BUTTON_WIDTH)
                    .on_press(MinusGamesGuiMessage::StopDownload)
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
