use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{
    DEFAULT_MODAL_BUTTON_WIDTH, GAME_CARD_ROW_HEIGHT, SMALL_MARGIN_DEFAULT,
};
use iced::widget::{button, image, row, text, Row};
use iced::{Center, Fill, Left};

#[derive(Clone, Debug, Default)]
pub struct GameCard {
    pub game: String,
    pub title: String,
    pub content: String,
    pub is_installed: bool,
    pub image: Option<image::Handle>,
}

// impl GameCard {

// pub(crate) fn with_image(game: String, image: String, is_installed: bool) -> GameCard {
//     Self {
//         game: game.clone(),
//         title: game,
//         content: "".to_string(),
//         is_installed,
//         image: fetch_image_sync(&image),
//     }
// }

// pub(crate) async fn with_image_async(
//     game: String,
//     image: String,
//     is_installed: bool,
// ) -> GameCard {
//     Self {
//         game: game.clone(),
//         title: game,
//         content: "".to_string(),
//         is_installed,
//         image: fetch_image(&image).await,
//     }
// }
// }

impl GameCard {
    pub(crate) fn new(game: String, content: String, is_installed: bool) -> Self {
        Self {
            game: game.clone(),
            title: game,
            content,
            is_installed,
            image: None,
        }
    }

    pub(crate) fn view(&self) -> Row<MinusGamesGuiMessage> {
        let row = match &self.image {
            None => {
                let mut row = row![
                    text(&self.title)
                        .width(Fill)
                        .shaping(text::Shaping::Advanced),
                    text(&self.content).width(Fill).align_x(Left),
                    // horizontal_space().width(Fill),
                    button(text("Play").width(Fill).align_x(Center))
                        .width(DEFAULT_MODAL_BUTTON_WIDTH)
                        .on_press(MinusGamesGuiMessage::Play(self.game.clone())),
                ]
                .align_y(Center);

                row = if self.is_installed {
                    row.push(
                        button(text("More").width(Fill).align_x(Center))
                            .width(DEFAULT_MODAL_BUTTON_WIDTH)
                            .on_press(MinusGamesGuiMessage::OpenGameModal(self.game.clone())),
                    )
                } else {
                    row.push(
                        button(text("Download").width(Fill).align_x(Center))
                            .width(DEFAULT_MODAL_BUTTON_WIDTH)
                            .on_press(MinusGamesGuiMessage::Repair(self.game.clone())),
                    )
                };
                row
            }
            Some(img) => {
                row![text(&self.title), image(img), button("Play")]
            }
        }
        .spacing(SMALL_MARGIN_DEFAULT);
        row.height(GAME_CARD_ROW_HEIGHT)
    }
}
