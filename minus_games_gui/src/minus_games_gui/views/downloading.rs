use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::MARGIN_DEFAULT;
use crate::minus_games_gui::views::components_helper::create_info_part;
use crate::minus_games_gui::views::game_helper::create_info_game_line;
use iced::widget::space::{horizontal, vertical};
use iced::widget::{Row, column, progress_bar, row, stack, text};
use iced::{Center, Color, Fill, Theme};

pub(crate) fn view(minus_games_gui: &MinusGamesGui) -> Row<'_, MinusGamesGuiMessage> {
    if let Some(cgi) = minus_games_gui.current_game.as_ref() {
        row![
            horizontal().width(MARGIN_DEFAULT),
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
                    .color(get_progress_bar_color(
                        minus_games_gui.files_downloaded,
                        minus_games_gui.files_to_download,
                        minus_games_gui.get_theme().unwrap()
                    ))
                    .width(Fill)
                    .height(Fill)
                    .center()
                ),
                vertical().height(MARGIN_DEFAULT),
                create_info_game_line(cgi),
            ]
            .align_x(Center),
            horizontal().width(MARGIN_DEFAULT)
        ]
    } else {
        row![
            horizontal(),
            column![
                vertical().height(MARGIN_DEFAULT),
                text("Downloading").size(50),
            ],
            horizontal(),
        ]
    }
}

fn get_progress_bar_color(
    files_downloaded: usize,
    files_to_download: usize,
    theme: Theme,
) -> impl Into<Color> {
    let switch_point = (files_to_download as f32 * 0.46) as usize;
    if switch_point < files_downloaded {
        theme.palette().primary.base.text
    } else {
        theme.palette().background.base.text
    }
}
