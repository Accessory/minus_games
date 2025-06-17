use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{
    GAME_CARD_IMAGE_HEIGHT, GAME_CARD_IMAGE_ROW_WIDTH, GAME_CARD_ROW_HEIGHT, READY_BUTTON_HEIGHT,
    READY_BUTTON_WIDTH, SMALL_MARGIN_DEFAULT, TEXT, TINY_MARGIN_DEFAULT,
};
use iced::ContentFit::Cover;
use iced::advanced::Widget;
use iced::widget::svg::Status;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{
    MouseArea, Row, button, center, column, container, horizontal_space, image, row, svg, text,
};
use iced::{Center, Element, Fill, Left, Right, Shrink, Theme, border, gradient};
use minus_games_models::game_infos::MinimalGameInfos;
use std::sync::LazyLock;

#[derive(Clone, Debug, Default)]
pub struct GameCard {
    pub game: String,
    pub title: String,
    // pub content: String,
    pub is_installed: bool,
    pub image: Option<image::Handle>,
    pub position: usize,
    pub is_on_server: bool,
    pub has_header: bool,
    pub minimal_game_infos: MinimalGameInfos,
}

static INSTALLED: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("./assets/svgs/installed.svg")));

static ON_SERVER: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("./assets/svgs/on-server.svg")));

static LINUX: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("./assets/svgs/linux.svg")));

static WINDOWS: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("./assets/svgs/windows.svg")));

impl GameCard {
    pub(crate) fn new(
        game: String,
        // content: String,
        is_installed: bool,
        position: usize,
        image: Option<image::Handle>,
        is_on_server: bool,
        has_header: bool,
        minimal_game_infos: MinimalGameInfos,
    ) -> Self {
        Self {
            game: game.clone(),
            title: game,
            // content,
            is_installed,
            image,
            position,
            is_on_server,
            has_header,
            minimal_game_infos,
        }
    }

    // pub(crate) fn get_shortened_title(&self) -> &str {
    //     if self.title.len() < 71 {
    //         self.title.as_str()
    //     } else {
    //         self.title.split_at(71).0
    //     }
    // }

    pub(crate) fn view(&self) -> MouseArea<'_, MinusGamesGuiMessage> {
        let row = self.create_row();

        MouseArea::new(row.height(GAME_CARD_ROW_HEIGHT))
            .on_enter(MinusGamesGuiMessage::EnterMouseArea(self.position))
    }

    fn create_image(&self) -> Element<'_, MinusGamesGuiMessage> {
        let element: Element<MinusGamesGuiMessage> = if let Some(image) = self.image.as_ref() {
            iced::widget::image(image)
                .content_fit(Cover)
                .height(GAME_CARD_IMAGE_HEIGHT)
                .width(GAME_CARD_IMAGE_ROW_WIDTH)
                .into()
        } else {
            center(
                text(self.title.chars().next().unwrap_or(' '))
                    .width(Fill)
                    .align_x(Center)
                    .shaping(Advanced),
            )
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                let style = container::Style::default();

                // style.text_color = Some(palette.secondary.strong.text);
                style
                    .border(border::color(palette.background.weak.color).width(1))
                    .background(
                        gradient::Linear::new(1.0)
                            .add_stop(0.0, palette.background.weak.color)
                            .add_stop(0.5, palette.background.strong.color)
                            .add_stop(1.0, palette.background.weak.color),
                    )
            })
            .width(GAME_CARD_IMAGE_ROW_WIDTH)
            .into()
        };
        element
    }

    fn create_os_part(&self) -> Row<'_, MinusGamesGuiMessage> {
        let mut items = Row::with_capacity(2);

        if self.minimal_game_infos.linux {
            items = items.push(svg(LINUX.clone()).style(Self::set_svg_style))
        }

        if self.minimal_game_infos.windows {
            items = items.push(svg(WINDOWS.clone()).style(Self::set_svg_style))
        }
        let width = TEXT * items.children().len() as u32;
        items.width(width)
    }

    fn create_installed_part(&self) -> Row<'_, MinusGamesGuiMessage> {
        let mut items = Row::with_capacity(2);

        if self.is_installed {
            items = items.push(svg(INSTALLED.clone()).style(Self::set_svg_style))
        }

        if self.is_on_server {
            items = items.push(svg(ON_SERVER.clone()).style(Self::set_svg_style))
        }

        let width = TEXT * items.children().len() as u32;
        items.width(width)
    }

    fn set_svg_style(theme: &Theme, _status: Status) -> svg::Style {
        let palette = theme.extended_palette();
        svg::Style {
            color: Some(palette.background.base.text),
        }
    }

    fn create_row(&self) -> Row<'_, MinusGamesGuiMessage> {
        let mut row = row![
            horizontal_space().width(TINY_MARGIN_DEFAULT),
            self.create_image(),
            row![
                text(&self.title)
                    .width(Fill)
                    .align_x(Left)
                    .shaping(Advanced)
                    .height(TEXT * 2),
                column![self.create_installed_part(), self.create_os_part(),]
                    .width(Shrink)
                    .align_x(Right)
            ],
            // Play
            button(
                text("") // Play
                    .height(READY_BUTTON_HEIGHT)
                    .width(TEXT)
                    .center()
                    .shaping(Advanced)
            )
            .width(READY_BUTTON_WIDTH)
            .on_press(MinusGamesGuiMessage::Play(self.game.clone())),
        ];

        row = if self.is_installed {
            row.push(
                // More
                button(
                    text("󰍜") // More
                        .height(READY_BUTTON_HEIGHT)
                        .width(Fill)
                        .center()
                        .shaping(Advanced),
                )
                .width(READY_BUTTON_WIDTH)
                .on_press(MinusGamesGuiMessage::OpenGameModal(
                    self.game.clone(),
                    self.is_on_server,
                )),
            )
        } else {
            row.push(
                // Download
                button(
                    text("") // Download
                        .height(READY_BUTTON_HEIGHT)
                        .width(Fill)
                        .center()
                        .shaping(Advanced),
                )
                .width(READY_BUTTON_WIDTH)
                .on_press(MinusGamesGuiMessage::Repair(self.game.clone())),
            )
        };
        row = row.push(horizontal_space().width(TINY_MARGIN_DEFAULT));
        row.spacing(SMALL_MARGIN_DEFAULT).align_y(Center)
    }
}

// fn color_from_string(input: &str) -> Color {
//     let mut hasher = DefaultHasher::new();
//     input.hash(&mut hasher);
//     let hash = hasher.finish();
//
//     // Extract color components.  This is a very basic mapping; you'll
//     // likely want to improve it (see more robust methods below).
//     let r = (hash & 0xFF) as u8;
//     let g = ((hash >> 8) & 0xFF) as u8;
//     let b = ((hash >> 16) & 0xFF) as u8;
//
//     Color::from_rgb8(r, g, b)
// }
