use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::messages::modal_callback::ModalCallback;
use crate::minus_games_gui::style_constants::{
    HALF_MARGIN_DEFAULT, LONG_BUTTON_WIDTH, MARGIN_DEFAULT, TEXT,
};
use crate::minus_games_gui::views::game_helper::create_info_game_line_with;
use crate::runtime::MODAL_SELECTED_OPTION;
use iced::widget::button::Status;
use iced::widget::space::vertical;
use iced::widget::{Column, Widget, button, center, container, mouse_area, opaque, text};
use iced::{Center, Color, Element, Fill, Theme};
use minus_games_client::runtime::{OFFLINE, get_config};
use std::sync::atomic::Ordering::Relaxed;

pub(crate) const MODAL_DELETE_BUTTON_ID: i8 = 0;
pub(crate) const MODAL_CONTINUE_DOWNLOAD_BUTTON_ID: i8 = 1;
pub(crate) const MODAL_REPAIR_BUTTON_ID: i8 = 1;
pub(crate) const MODAL_OPEN_FOLDER_BUTTON_ID: i8 = 2;
pub(crate) const MODAL_OPEN_GAME_INFOS: i8 = 3;
pub(crate) const MODAL_CLOSE_BUTTON_ID: i8 = 4;

pub(crate) fn create_modal(
    game: &str,
    is_on_server: bool,
    width: f32,
) -> Element<'_, MinusGamesGuiMessage> {
    let game_infos_option = get_config().get_game_infos(game);
    let mut column: Column<Element<'_, MinusGamesGuiMessage>> = Column::new();
    column = column.push(
        text(game)
            .size(TEXT)
            .height(TEXT * 2 + HALF_MARGIN_DEFAULT)
            .shaping(text::Shaping::Auto)
            .boxed(),
    );
    if let Some(game_infos) = &game_infos_option {
        // column = column.push(text(format!("Engine: {}", game_infos.engine)));
        // column = column.push(text(format!(
        //     "Linux support: {}",
        //     game_infos.supported_platforms.linux
        // )));
        // column = column.push(text(format!(
        //     "Windows support: {}",
        //     game_infos.supported_platforms.windows
        // )));
        column = column.push(
            create_info_game_line_with(
                game_infos.engine.to_string(),
                game_infos.supports_linux(),
                game_infos.supports_windows(),
            )
            .boxed(),
        );
        column = column.push(vertical().height(MARGIN_DEFAULT).boxed());
        column = column.push(
            button(text("Delete").width(Fill).align_x(Center).boxed())
                .style(set_modal_style(MODAL_DELETE_BUTTON_ID))
                .width(LONG_BUTTON_WIDTH)
                .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                    ModalCallback::DeleteGame(game.to_string()),
                )))
                .boxed(),
        );
        if !OFFLINE.load(Relaxed) && is_on_server {
            column = column.push(vertical().height(HALF_MARGIN_DEFAULT).boxed());
            if get_config().is_game_dirty(game) {
                column = column.push(
                    button(
                        text("Continue Download")
                            .width(Fill)
                            .align_x(Center)
                            .boxed(),
                    )
                    .style(set_modal_style(MODAL_CONTINUE_DOWNLOAD_BUTTON_ID))
                    .width(LONG_BUTTON_WIDTH)
                    .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                        ModalCallback::RepairGame(game.to_string()),
                    )))
                    .boxed(),
                );
            } else {
                column = column.push(
                    button(text("Repair").width(Fill).align_x(Center).boxed())
                        .style(set_modal_style(MODAL_REPAIR_BUTTON_ID))
                        .width(LONG_BUTTON_WIDTH)
                        .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                            ModalCallback::RepairGame(game.to_string()),
                        )))
                        .boxed(),
                );
            }
        }
        column = column.push(vertical().height(HALF_MARGIN_DEFAULT).boxed());
        column = column.push(
            button(text("Open folder").width(Fill).align_x(Center).boxed())
                .style(set_modal_style(MODAL_OPEN_FOLDER_BUTTON_ID))
                .width(LONG_BUTTON_WIDTH)
                .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                    ModalCallback::OpenGameFolder(get_config().get_game_path(game)),
                )))
                .boxed(),
        );
        column = column.push(vertical().height(HALF_MARGIN_DEFAULT).boxed());
        column = column.push(
            button(text("Open Game Infos").width(Fill).align_x(Center).boxed())
                .style(set_modal_style(MODAL_OPEN_GAME_INFOS))
                .width(LONG_BUTTON_WIDTH)
                .on_press(MinusGamesGuiMessage::ModalCallback(Some(
                    ModalCallback::OpenGameFolder(get_config().get_game_infos_path_from_game(game)),
                )))
                .boxed(),
        );
    }
    column = column.push(vertical().height(MARGIN_DEFAULT).boxed());
    column = column.push(
        button(text("Close").width(Fill).align_x(Center).boxed())
            .style(set_modal_style(MODAL_CLOSE_BUTTON_ID))
            .width(LONG_BUTTON_WIDTH)
            .on_press(MinusGamesGuiMessage::ModalCallback(None))
            .boxed(),
    );
    column = column.push(vertical().height(HALF_MARGIN_DEFAULT).boxed());
    column = column.align_x(Center);

    let modal_content = container(column.boxed())
        .style(container::bordered_box)
        .padding(HALF_MARGIN_DEFAULT as u16)
        .align_x(Center)
        .align_y(Center)
        .width(width * 0.66);

    opaque(
        mouse_area(
            center(opaque(modal_content.boxed()).boxed())
                .style(|_theme| container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                })
                .boxed(),
        )
        .on_press(MinusGamesGuiMessage::ModalCallback(None))
        .boxed(),
    )
    .boxed()
}

fn set_modal_style(modal_button_id: i8) -> impl Fn(&Theme, Status) -> button::Style {
    move |theme: &Theme, status| {
        if status == Status::Hovered {
            MODAL_SELECTED_OPTION.store(modal_button_id, Relaxed);
        }

        let final_status = if MODAL_SELECTED_OPTION.load(Relaxed) == modal_button_id {
            Status::Hovered
        } else {
            status
        };

        button::primary(theme, final_status)
    }
}
