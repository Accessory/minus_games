use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::style_constants::{
    DEFAULT_MODAL_BUTTON_WIDTH, GAME_CARD_ROW_HEIGHT, SMALL_MARGIN_DEFAULT,
};
use iced::widget::{button, image, row, text, MouseArea};
use iced::{Center, Fill, Left};

#[derive(Clone, Debug, Default)]
pub struct GameCard {
    pub game: String,
    pub title: String,
    pub content: String,
    pub is_installed: bool,
    pub image: Option<image::Handle>,
    pub position: usize,
    pub is_on_server: bool,
}

impl GameCard {
    pub(crate) fn new(
        game: String,
        content: String,
        is_installed: bool,
        position: usize,
        is_on_server: bool,
    ) -> Self {
        Self {
            game: game.clone(),
            title: game,
            content,
            is_installed,
            image: None,
            position,
            is_on_server,
        }
    }

    pub(crate) fn view(&self) -> MouseArea<MinusGamesGuiMessage> {
        let row = match &self.image {
            None => {
                let mut row = row![
                    text(&self.title)
                        .width(Fill)
                        .shaping(text::Shaping::Advanced),
                    text(&self.content).width(Fill).align_x(Left),
                    button(text("Play").width(Fill).align_x(Center))
                        .width(DEFAULT_MODAL_BUTTON_WIDTH)
                        .on_press(MinusGamesGuiMessage::Play(self.game.clone())),
                ]
                .align_y(Center);

                row = if self.is_installed {
                    row.push(
                        button(text("More").width(Fill).align_x(Center))
                            .width(DEFAULT_MODAL_BUTTON_WIDTH)
                            .on_press(MinusGamesGuiMessage::OpenGameModal(
                                self.game.clone(),
                                self.is_on_server,
                            )),
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

        MouseArea::new(row.height(GAME_CARD_ROW_HEIGHT))
            .on_enter(MinusGamesGuiMessage::EnterMouseArea(self.position))
    }
}
