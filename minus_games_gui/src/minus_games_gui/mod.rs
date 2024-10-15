use crate::minus_games_gui::game_card::GameCard;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::messages::modal_callback::ModalCallback;
use crate::minus_games_gui::minus_games_settings::MinusGamesSettings;
use crate::minus_games_gui::settings::{handle_change_event, save_new_settings};
use crate::minus_games_gui::style_constants::{
    DOUBLE_MARGIN_DEFAULT, HALF_MARGIN_DEFAULT, MARGIN_DEFAULT, SPACING_DEFAULT, TEXT, TOP_BUTTON,
};
use crate::minus_games_gui::views::game_info_modal::create_modal;
use crate::minus_games_gui::views::{downloading, gaming, loading, ready, settings_view};
use crate::runtime::get_gui_config;
use iced::futures::{SinkExt, Stream};
use iced::widget::scrollable::Anchor::Start;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{
    button, column, horizontal_space, row, scrollable, stack, text, text_input, Column,
};
use iced::{
    event, stream, window, Center, Element, Event, Fill, Length, Size, Subscription, Task, Theme,
};
use minus_games_client::actions::delete::delete_game;
use minus_games_client::actions::repair::repair_game;
use minus_games_client::actions::run::run_game_synced;
use minus_games_client::runtime::{
    get_config, get_installed_games, send_event, set_sender, MinusGamesClientEvents, CLIENT,
};
use minus_games_models::game_infos::GameInfos;
use settings::override_config;
use tracing::{debug, info};

pub mod configuration;
mod game_card;
mod messages;
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
pub struct MinusGamesGui {
    pub theme: Theme,
    pub game_cards: Vec<GameCard>,
    pub state: MinusGamesState,
    pub files_to_download: usize,
    pub files_downloaded: usize,
    pub current_game: Option<GameInfos>,
    pub current_game_name: Option<String>,
    pub settings: Option<MinusGamesSettings>,
    pub filter: String,
    pub model: Option<String>,
    pub size: Size,
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
            if get_gui_config().fullscreen {
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
        if let Some(theme_string) = get_gui_config().theme.as_ref() {
            if let Some(theme) = Theme::ALL
                .iter()
                .find(|&t| theme_string == t.to_string().as_str())
            {
                return (
                    MinusGamesGui {
                        theme: theme.clone(),
                        ..MinusGamesGui::default()
                    },
                    Self::start_async_init(),
                );
            }
        }
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
        // send_event("Create Cards".into()).await;
        debug!("Create Cards");
        let installed_games = get_installed_games();
        let server_games = CLIENT.get_games_list().await.unwrap_or_default();

        let mut rtn = Vec::new();
        for game in &installed_games {
            let content = if server_games.contains(game) { "Installed/On Server" } else { "Installed" };
            let game_card = GameCard::new(game.to_string(), content.into(), true);
            rtn.push(game_card);
        }

        for game in &server_games {
            if installed_games.contains(game) {
                continue;
            }
            let game_card = GameCard::new(game.to_string(), "On Server".into(), false);
            rtn.push(game_card);
        }

        rtn
    }

    pub fn update(&mut self, message: MinusGamesGuiMessage) -> Task<MinusGamesGuiMessage> {
        match message {
            MinusGamesGuiMessage::Loading => {}
            MinusGamesGuiMessage::Reload => {
                self.game_cards.clear();
                self.state = MinusGamesState::Loading;
                return Self::load();
            }
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
            MinusGamesGuiMessage::Repair(game) => {
                self.state = MinusGamesState::Loading;
                return Task::perform(
                    async move {
                        repair_game(&game).await;
                    },
                    MinusGamesGuiMessage::FinishedRepairing,
                );
            }
            MinusGamesGuiMessage::OpenGameModal(game) => {
                self.model = Some(game);
            }
            MinusGamesGuiMessage::ModalCallback(message_option) => {
                self.model = None;
                if let Some(message) = message_option {
                    return match message {
                        ModalCallback::DeleteGame(game) => {
                            Task::done(MinusGamesGuiMessage::Delete(game))
                        }
                        ModalCallback::RepairGame(game) => {
                            Task::done(MinusGamesGuiMessage::Repair(game))
                        }
                        ModalCallback::OpenGameFolder(path) => {
                            open::that(path).ok();
                            Task::none()
                        }
                    };
                }
            }
            MinusGamesGuiMessage::FinishedPlay(_)
            | MinusGamesGuiMessage::FinishedDelete(_)
            | MinusGamesGuiMessage::FinishedRepairing(_) => {
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
            MinusGamesGuiMessage::CurrentGame(game) => {
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
                if let Event::Window(window::Event::Resized(size)) = event {
                    self.size = size;
                    return Task::none();
                }
            }
            MinusGamesGuiMessage::Fullscreen => {
                return window::get_latest()
                    .and_then(move |window| window::change_mode(window, window::Mode::Fullscreen));
            }
            MinusGamesGuiMessage::Settings => {
                self.settings = Some(MinusGamesSettings::from_config_with_theme(
                    get_config(),
                    self.get_theme(),
                ));
                self.state = MinusGamesState::Settings;
            }
            MinusGamesGuiMessage::ApplyScreenSettings => {
                return if get_gui_config().fullscreen {
                    window::get_latest().and_then(move |window| {
                        window::change_mode(window, window::Mode::Fullscreen)
                    })
                } else {
                    window::get_latest()
                        .and_then(move |window| window::change_mode(window, window::Mode::Windowed))
                }
            }
            MinusGamesGuiMessage::BackFromSettings(save) => {
                if save {
                    save_new_settings(self.settings.as_ref());
                    override_config(self.settings.as_ref());
                    if let Some(settings) = self.settings.take() {
                        self.theme = settings.theme;
                    }
                    // return Task::perform(Self::close(), MinusGamesGuiMessage::CloseApplication);
                } else if let Some(settings) = self.settings.take() {
                    self.theme = settings.initial_theme;
                }
                self.state = MinusGamesState::Loading;
                return Task::batch([
                    Task::done(MinusGamesGuiMessage::ApplyScreenSettings),
                    Self::load(),
                ]);
            }
            MinusGamesGuiMessage::ChangeSetting(change_input) => {
                handle_change_event(self.settings.as_mut(), change_input);
                if let Some(settings) = self.settings.as_ref() {
                    if self.theme != settings.theme {
                        self.theme = settings.theme.clone();
                    }
                }
            }
            MinusGamesGuiMessage::FilterChanged(change) => {
                self.filter = change;
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

        let content = scrollable(to_display.height(Length::Shrink))
            .direction(Direction::Vertical(
                Scrollbar::new()
                    .width(5)
                    .margin(5)
                    .scroller_width(5)
                    .anchor(Start),
            ))
            .width(Length::Fill)
            .height(Length::Fill);

        match &self.model {
            None => content.into(),
            Some(game) => stack!(content, create_modal(game, self.size.width).into()).into(),
        }
    }

    fn create_ready_view(&self) -> Column<MinusGamesGuiMessage> {
        if self.game_cards.is_empty() {
            return column![row![
                text("No Games found...").size(50),
                horizontal_space().width(Fill),
                button("Reload")
                    .on_press(MinusGamesGuiMessage::Reload)
                    .padding(TOP_BUTTON),
                horizontal_space().width(HALF_MARGIN_DEFAULT),
                button("Settings")
                    .on_press(MinusGamesGuiMessage::Settings)
                    .padding(TOP_BUTTON),
                horizontal_space().width(MARGIN_DEFAULT),
                button("Quit")
                    .on_press(MinusGamesGuiMessage::CloseApplication(()))
                    .padding(TOP_BUTTON)
            ]];
        }
        let mut rtn = Column::with_capacity(self.game_cards.len() + 1).spacing(SPACING_DEFAULT);
        rtn = rtn.push(
            row![
                column![
                    text("Games").size(TEXT),
                    text_input("Filter", &self.filter)
                        .on_input(MinusGamesGuiMessage::FilterChanged)
                        .width(Fill)
                ],
                horizontal_space().width(DOUBLE_MARGIN_DEFAULT),
                button("Reload")
                    .on_press(MinusGamesGuiMessage::Reload)
                    .padding(TOP_BUTTON),
                horizontal_space().width(HALF_MARGIN_DEFAULT),
                button("Settings")
                    .on_press(MinusGamesGuiMessage::Settings)
                    .padding(TOP_BUTTON),
                horizontal_space().width(MARGIN_DEFAULT),
                button("Quit")
                    .on_press(MinusGamesGuiMessage::CloseApplication(()))
                    .padding(TOP_BUTTON)
            ]
            .align_y(Center),
        );
        for (i, game_card) in self.game_cards.iter().enumerate() {
            if game_card
                .game
                .to_lowercase()
                .contains(self.filter.trim().to_lowercase().as_str())
            {
                rtn = rtn.push(
                    row![text(format!("{i:0>3}")), game_card.view()].spacing(SPACING_DEFAULT),
                );
            }
        }

        rtn
    }

    pub fn get_theme(&self) -> Theme {
        self.theme.clone()
    }
}
