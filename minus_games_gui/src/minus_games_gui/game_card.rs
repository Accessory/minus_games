use crate::minus_games_gui::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::utils::{fetch_image, fetch_image_sync};
use iced::widget::{button, horizontal_space, image, row, text, Row};
use iced_wgpu::core::Length;

#[derive(Clone, Debug, Default)]
pub struct GameCard {
    pub game: String,
    pub title: String,
    pub content: String,
    pub is_installed: bool,
    pub image: Option<image::Handle>,
}

impl GameCard {
    #[allow(dead_code)]
    pub(crate) fn with_image(game: String, image: String, is_installed: bool) -> GameCard {
        Self {
            game: game.clone(),
            title: game,
            content: "".to_string(),
            is_installed,
            image: fetch_image_sync(&image),
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn with_image_async(
        game: String,
        image: String,
        is_installed: bool,
    ) -> GameCard {
        Self {
            game: game.clone(),
            title: game,
            content: "".to_string(),
            is_installed,
            image: fetch_image(&image).await,
        }
    }
}

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
        let element = match &self.image {
            None => {
                let mut row = row![
                    text(&self.title).width(Length::Fill),
                    text(&self.content),
                    horizontal_space(),
                    button("Play").on_press(MinusGamesGuiMessage::Play(self.game.clone())),
                ];

                if self.is_installed {
                    row = row.push(
                        button("Delete").on_press(MinusGamesGuiMessage::Delete(self.game.clone())),
                    );
                }
                row
            }
            Some(img) => {
                row![text(&self.title), image(img), button("Play")]
            }
        }
        .spacing(10);
        element
    }
}
