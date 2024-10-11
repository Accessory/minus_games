use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::messages::modal_callback::ModalCallback;
use crate::minus_games_gui::style_constants::{
    DEFAULT_MODAL_BUTTON_WIDTH, HALF_MARGIN_DEFAULT, MARGIN_DEFAULT,
};
use iced::widget::{button, center, container, mouse_area, opaque, text, vertical_space, Column};
use iced::{Center, Color, Element};
use minus_games_client::runtime::get_config;

pub(crate) fn create_modal(game: &str, width: f32) -> impl Into<Element<MinusGamesGuiMessage>> {
    let game_infos_option = get_config().get_game_infos(game);
    let mut column = Column::new();
    column = column.push(text(game).size(24));
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
        column = column.push(text(format!(
            "Linux support: {}",
            game_infos.supported_platforms.linux
        )));
        column = column.push(vertical_space().height(MARGIN_DEFAULT));
        column = column.push(button("Delete").width(DEFAULT_MODAL_BUTTON_WIDTH).on_press(
            MinusGamesGuiMessage::ModalCallback(Some(ModalCallback::DeleteGame(game.to_string()))),
        ));
        column = column.push(vertical_space().height(HALF_MARGIN_DEFAULT));
        column = column.push(button("Repair").width(DEFAULT_MODAL_BUTTON_WIDTH).on_press(
            MinusGamesGuiMessage::ModalCallback(Some(ModalCallback::RepairGame(game.to_string()))),
        ));
        column = column.push(vertical_space().height(HALF_MARGIN_DEFAULT));
        column = column.push(
            button("Open folder")
                .width(DEFAULT_MODAL_BUTTON_WIDTH)
                .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                    ModalCallback::OpenGameFolder(get_config().get_game_path(game)),
                ))),
        );
    }
    column = column.push(vertical_space().height(MARGIN_DEFAULT));
    column = column.push(
        button("Close")
            .width(DEFAULT_MODAL_BUTTON_WIDTH)
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
