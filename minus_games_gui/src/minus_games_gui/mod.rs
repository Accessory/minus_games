use crate::minus_games_gui::game_card::GameCard;
use crate::minus_games_gui::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::minus_games_settings::MinusGamesSettings;
use crate::minus_games_gui::settings::{handle_change_event, save_new_settings};
use crate::minus_games_gui::style_constants::{
    BIG_TEXT, MARGIN_DEFAULT, SPACING_DEFAULT, TOP_BUTTON,
};
use crate::minus_games_gui::views::{downloading, gaming, loading, ready, settings_view};
use iced::futures::{SinkExt, Stream};
use iced::widget::scrollable::Anchor::Start;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{
    button, column, horizontal_space, row, scrollable, text, vertical_space, Column,
};
use iced::{event, stream, window, Element, Event, Length, Subscription, Task};
use minus_games_client::actions::delete::delete_game;
use minus_games_client::actions::run::run_game_synced;
use minus_games_client::runtime::{
    get_config, get_installed_games, send_event, set_sender, MinusGamesClientEvents, CLIENT,
};
use minus_games_models::game_infos::GameInfos;
use settings::override_config;
use tracing::info;

mod game_card;
mod minus_games_gui_message;
mod minus_games_settings;
mod settings;
mod style_constants;
mod utils;
mod views;

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum MinusGamesState {
    #[default]
    Loading,
    Ready,
    Downloading,
    Gaming,
    Settings,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct MinusGamesGui {
    pub game_cards: Vec<GameCard>,
    #[allow(dead_code)]
    pub state: MinusGamesState,
    pub files_to_download: usize,
    pub files_downloaded: usize,
    pub current_game: Option<GameInfos>,
    pub current_game_name: Option<String>,
    pub settings: Option<MinusGamesSettings>,
}

impl MinusGamesGui {
    pub fn batch_subscription(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::batch([
            self.event_subscription(),
            self.on_close(),
            self.start_fullscreen(),
        ])
    }

    pub fn start_fullscreen(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::run(Self::set_start_fullscreen)
    }

    pub fn set_start_fullscreen() -> impl Stream<Item = MinusGamesGuiMessage> {
        stream::channel(1, |mut output| async move {
            if std::env::var("MINUS_GAMES_GUI_FULLSCREEN").is_ok_and(|i| i == "true") {
                let _ = output.send(MinusGamesGuiMessage::Fullscreen).await;
            }
        })
    }

    pub fn event_subscription(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::run(Self::run_events)
    }

    pub fn on_close(&self) -> Subscription<MinusGamesGuiMessage> {
        event::listen().map(MinusGamesGuiMessage::WindowEvent)
    }

    pub fn run_events() -> impl Stream<Item = MinusGamesGuiMessage> {
        stream::channel(24, |mut output| async move {
            let (sender, mut receiver) =
                tokio::sync::mpsc::channel(std::thread::available_parallelism().unwrap().get());
            set_sender(sender).await;
            while let Some(event) = receiver.recv().await {
                let _ = output.send(event.into()).await;
            }
        })
    }

    pub fn init() -> (Self, Task<MinusGamesGuiMessage>) {
        (MinusGamesGui::default(), Self::start_async_init())
    }

    pub fn start_async_init() -> Task<MinusGamesGuiMessage> {
        Task::perform(
            MinusGamesGui::async_init(),
            MinusGamesGuiMessage::InitComplete,
        )
    }

    // pub async fn async_init() -> Arc<RwLock<tokio::sync::mpsc::Receiver<MinusGamesClientEvents>>> {
    pub async fn async_init() {
        // let (rx, tx) = tokio::sync::mpsc::channel(std::thread::available_parallelism().unwrap().get());
        // set_sender(rx.into()).await;
        // Arc::new(RwLock::new(tx))
        info!("Async Init!");
    }

    pub fn load() -> Task<MinusGamesGuiMessage> {
        Task::perform(MinusGamesGui::create_cards(), MinusGamesGuiMessage::Created)
    }

    pub async fn create_cards() -> Vec<GameCard> {
        send_event("Create Cards".into()).await;
        let installed_games = get_installed_games();
        let server_games = CLIENT.get_games_list().await.unwrap_or_default();

        let mut rtn = Vec::new();
        for game in &installed_games {
            let game_card = GameCard::new(game.to_string(), "Is Installed".into(), true);
            rtn.push(game_card);
        }

        for game in &server_games {
            if installed_games.contains(game) {
                continue;
            }
            let game_card = GameCard::new(game.to_string(), "Is not Installed".into(), false);
            rtn.push(game_card);
        }

        rtn
    }

    pub fn update(&mut self, message: MinusGamesGuiMessage) -> Task<MinusGamesGuiMessage> {
        match message {
            MinusGamesGuiMessage::Loading => {}
            MinusGamesGuiMessage::Created(game_cards) => {
                self.game_cards = game_cards;
                self.state = MinusGamesState::Ready;
            }
            MinusGamesGuiMessage::Play(game) => {
                self.files_to_download = 100;
                self.files_downloaded = 0;
                return Task::perform(
                    async move {
                        run_game_synced(&game).await;
                    },
                    MinusGamesGuiMessage::FinishedPlay,
                );
            }
            MinusGamesGuiMessage::Delete(game) => {
                return Task::perform(
                    async move {
                        delete_game(&game, false);
                    },
                    MinusGamesGuiMessage::FinishedDelete,
                );
            }
            MinusGamesGuiMessage::FinishedPlay(_) | MinusGamesGuiMessage::FinishedDelete(_) => {
                self.state = MinusGamesState::Ready;
                return MinusGamesGui::load();
            } // _ => Task::none(),
            MinusGamesGuiMessage::Init => {}
            MinusGamesGuiMessage::InitComplete(_) => return Self::load(),
            MinusGamesGuiMessage::SetFilesToDownload(files_count) => {
                self.state = MinusGamesState::Downloading;
                self.files_downloaded = 0;
                self.files_to_download = files_count;
            }
            MinusGamesGuiMessage::FinishedDownloading => {
                self.files_downloaded += 1;
            }
            MinusGamesGuiMessage::LogMessage(msg) => {
                info!("{msg}");
            }
            MinusGamesGuiMessage::LogStaticMessage(msg) => {
                info!("{msg}");
            }
            MinusGamesGuiMessage::Noop => {}
            MinusGamesGuiMessage::SyncFileInfosComplete => {
                self.state = MinusGamesState::Downloading;
                if let Some(game) = &self.current_game_name {
                    self.current_game = get_config().get_game_infos(game);
                }
            }
            MinusGamesGuiMessage::RunningGame(game) => {
                self.current_game_name = Some(game);
            }
            MinusGamesGuiMessage::StartGame(_) => {
                if self.current_game.is_none() {
                    if let Some(game) = &self.current_game_name {
                        self.current_game = get_config().get_game_infos(game);
                    }
                }

                self.state = MinusGamesState::Gaming;
            }
            MinusGamesGuiMessage::CloseGame(_) => {
                self.current_game = None;
                self.current_game_name = None;
                self.state = MinusGamesState::Loading;
                return Self::load();
            }
            MinusGamesGuiMessage::Exit => {
                return Task::perform(Self::close(), MinusGamesGuiMessage::CloseApplication)
            }

            MinusGamesGuiMessage::CloseApplication(_) => {
                info!("Client event listener closed!");
                return window::get_latest().and_then(window::close);
            }
            MinusGamesGuiMessage::WindowEvent(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    info!("Close Application!");
                    return Task::perform(Self::close(), MinusGamesGuiMessage::CloseApplication);
                }
            }
            MinusGamesGuiMessage::Fullscreen => {
                return window::get_latest()
                    .and_then(move |window| window::change_mode(window, window::Mode::Fullscreen));
            }
            MinusGamesGuiMessage::Settings => {
                self.settings = Some(MinusGamesSettings::from_config(get_config()));
                self.state = MinusGamesState::Settings;
            }
            MinusGamesGuiMessage::BackFromSettings(save) => {
                if save {
                    save_new_settings(self.settings.as_ref());
                    override_config(self.settings.as_ref());
                    // return Task::perform(Self::close(), MinusGamesGuiMessage::CloseApplication);
                }
                self.settings = None;
                self.state = MinusGamesState::Loading;
                return Self::load();
            }
            MinusGamesGuiMessage::ChangeSetting(change_input) => {
                handle_change_event(self.settings.as_mut(), change_input);
            }
        };
        Task::none()
    }

    async fn close() {
        send_event(MinusGamesClientEvents::Close).await;
    }

    pub fn view(&self) -> Element<MinusGamesGuiMessage> {
        let to_display = match self.state {
            MinusGamesState::Loading => loading::view(),
            MinusGamesState::Ready => ready::view(self),
            MinusGamesState::Downloading => downloading::view(self),
            MinusGamesState::Gaming => gaming::view(self),
            MinusGamesState::Settings => settings_view::view(self),
        };

        scrollable(to_display.height(Length::Shrink))
            .direction(Direction::Vertical(
                Scrollbar::new()
                    .width(5)
                    .margin(5)
                    .scroller_width(5)
                    .anchor(Start),
            ))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn create_ready_view(&self) -> Column<MinusGamesGuiMessage> {
        if self.game_cards.is_empty() {
            return column![
                text("No Games found...").size(50),
                vertical_space().height(MARGIN_DEFAULT),
                button("Settings").on_press(MinusGamesGuiMessage::Settings)
            ];
        }
        let mut rtn = Column::with_capacity(self.game_cards.len() + 1).spacing(SPACING_DEFAULT);
        rtn = rtn.push(row![
            text("Games").size(BIG_TEXT),
            horizontal_space(),
            button("Settings")
                .on_press(MinusGamesGuiMessage::Settings)
                .padding(TOP_BUTTON),
            horizontal_space().width(MARGIN_DEFAULT),
            button("Quit")
                .on_press(MinusGamesGuiMessage::CloseApplication(()))
                .padding(TOP_BUTTON)
        ]);
        for (i, game_card) in self.game_cards.iter().enumerate() {
            rtn =
                rtn.push(row![text(format!("{i:0>3}")), game_card.view()].spacing(SPACING_DEFAULT));
        }

        rtn
    }
}
