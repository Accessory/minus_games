use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::messages::modal_callback::ModalCallback;
use crate::minus_games_gui::style_constants::{
    HALF_MARGIN_DEFAULT, LONG_BUTTON_WIDTH, MARGIN_DEFAULT,
};
use iced::widget::{button, center, container, mouse_area, opaque, text, vertical_space, Column};
use iced::{Center, Color, Element, Fill};
use minus_games_client::runtime::{get_config, OFFLINE};
use std::sync::atomic::Ordering::Relaxed;

pub(crate) fn create_modal(game: &str, width: f32) -> impl Into<Element<MinusGamesGuiMessage>> {
    let game_infos_option = get_config().get_game_infos(game);
    let mut column = Column::new();
    column = column.push(text(game).size(24).shaping(text::Shaping::Advanced));
    if let Some(game_infos) = &game_infos_option {
        column = column.push(text(format!("Engine: {}", game_infos.engine)));
        column = column.push(text(format!(
            "Linux support: {}",
            game_infos.supported_platforms.linux
        )));
        column = column.push(text(format!(
            "Windows support: {}",
            game_infos.supported_platforms.windows
        )));
        column = column.push(vertical_space().height(MARGIN_DEFAULT));
        column = column.push(
            button(text("Delete").width(Fill).align_x(Center))
                .width(LONG_BUTTON_WIDTH)
                .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                    ModalCallback::DeleteGame(game.to_string()),
                ))),
        );
        column = column.push(vertical_space().height(HALF_MARGIN_DEFAULT));
        if !OFFLINE.load(Relaxed) {
            if get_config().is_game_dirty(game) {
                column = column.push(
                    button(text("Continue Download").width(Fill).align_x(Center))
                        .width(LONG_BUTTON_WIDTH)
                        .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                            ModalCallback::RepairGame(game.to_string()),
                        ))),
                );
            } else {
                column = column.push(
                    button(text("Repair").width(Fill).align_x(Center))
                        .width(LONG_BUTTON_WIDTH)
                        .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                            ModalCallback::RepairGame(game.to_string()),
                        ))),
                );
            }
        }
        column = column.push(vertical_space().height(HALF_MARGIN_DEFAULT));
        column = column.push(
            button(text("Open folder").width(Fill).align_x(Center))
                .width(LONG_BUTTON_WIDTH)
                .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                    ModalCallback::OpenGameFolder(get_config().get_game_path(game)),
                ))),
        );
    }
    column = column.push(vertical_space().height(MARGIN_DEFAULT));
    column = column.push(
        button(text("Close").width(Fill).align_x(Center))
            .width(LONG_BUTTON_WIDTH)
            .on_press(MinusGamesGuiMessage::ModalCallback(None)),
    );
    column = column.push(vertical_space().height(HALF_MARGIN_DEFAULT));
    column = column.align_x(Center);

    let modal_content = container(column)
        .style(container::bordered_box)
        .padding(HALF_MARGIN_DEFAULT)
        .align_x(Center)
        .align_y(Center)
        .width(width * 0.66);

    opaque(
        mouse_area(center(opaque(modal_content)).style(|_theme| {
            container::Style {
                background: Some(
                    Color {
                        a: 0.8,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..container::Style::default()
            }
        }))
        .on_press(MinusGamesGuiMessage::ModalCallback(None)),
    )
}
