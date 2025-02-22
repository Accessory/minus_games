use crate::minus_games_gui::game_card::GameCard;
use crate::minus_games_gui::handlers::gamepad_input_handler::gamepad_input_handler;
use crate::minus_games_gui::handlers::keyboard_event_handler::handle_keyboard_events;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::messages::modal_callback::ModalCallback;
use crate::minus_games_gui::minus_games_settings::MinusGamesSettings;
use crate::minus_games_gui::settings::{handle_change_event, save_new_settings};
use crate::minus_games_gui::style_constants::{
    DOUBLE_MARGIN_DEFAULT, GAME_CARD_ROW_HEIGHT, HALF_MARGIN_DEFAULT, MARGIN_DEFAULT,
    SPACING_DEFAULT, TEXT, TOP_BUTTON,
};
use crate::minus_games_gui::views::game_info_modal::create_modal;
use crate::minus_games_gui::views::{downloading, gaming, loading, ready, settings_view};
use crate::minus_games_gui::widgets::always_highlighter::AlwaysHighlighter;
use crate::runtime::{CLOSING, SCROLLABLE_ID, get_gui_config};
use iced::futures::{SinkExt, Stream};
use iced::widget::scrollable::Anchor::Start;
use iced::widget::scrollable::{AbsoluteOffset, Direction, Scrollbar};
use iced::widget::{
    Column, button, column, horizontal_space, row, scrollable, stack, text, text_input,
    vertical_space,
};
use iced::{Center, Element, Fill, Length, Size, Subscription, Task, Theme, event, stream, window};
use minus_games_client::actions::delete::delete_game;
use minus_games_client::actions::repair::{repair_all_games, repair_game};
use minus_games_client::actions::run::run_game_synced;
use minus_games_client::actions::scan::scan_for_games;
use minus_games_client::runtime::{
    MinusGamesClientEvents, STOP_DOWNLOAD, get_client, get_config, get_installed_games,
    kill_current_running_game, reset_client, send_event, set_sender,
};
use minus_games_models::game_infos::GameInfos;
use settings::override_config;
use std::cmp;
use std::sync::atomic::Ordering::Relaxed;
use tracing::{debug, info};

pub mod configuration;
mod game_card;
mod handlers;
mod messages;
mod minus_games_settings;
mod settings;
mod style_constants;
mod views;
mod widgets;

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
pub(crate) struct MinusGamesGui {
    pub theme: Theme,
    pub game_cards: Vec<GameCard>,
    pub state: MinusGamesState,
    pub files_to_download: usize,
    pub files_downloaded: usize,
    pub current_game: Option<GameInfos>,
    pub current_game_name: Option<String>,
    pub settings: Option<MinusGamesSettings>,
    pub filter: String,
    pub model: Option<(String, bool)>,
    pub size: Size,
    pub highlight_map: Vec<usize>,
    pub scroll_offset: AbsoluteOffset,
    pub current_highlight_position: usize,
    pub went_up: bool,
    pub last_input_mouse: bool,
    pub block_highlighting: bool,
}

const FILTER_ID: &str = "FILTER_ID";

impl MinusGamesGui {
    pub(crate) fn batch_subscription(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::batch([
            self.event_subscription(),
            self.on_close(),
            self.start_fullscreen(),
            self.gamepad_input(),
        ])
    }

    pub(crate) fn start_fullscreen(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::run(Self::set_start_fullscreen)
    }

    pub(crate) fn set_start_fullscreen() -> impl Stream<Item = MinusGamesGuiMessage> {
        stream::channel(1, |mut output| async move {
            if get_gui_config().fullscreen {
                let _ = output.send(MinusGamesGuiMessage::Fullscreen).await;
            }
        })
    }

    pub(crate) fn gamepad_input(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::run(gamepad_input_handler)
    }

    pub(crate) fn event_subscription(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::run(Self::run_events)
    }

    pub(crate) fn on_close(&self) -> Subscription<MinusGamesGuiMessage> {
        event::listen().map(MinusGamesGuiMessage::Event)
    }

    pub(crate) fn run_events() -> impl Stream<Item = MinusGamesGuiMessage> {
        stream::channel(24, |mut output| async move {
            let (sender, mut receiver) =
                tokio::sync::mpsc::channel(std::thread::available_parallelism().unwrap().get());
            set_sender(sender).await;
            while let Some(event) = receiver.recv().await {
                let _ = output.send(event.into()).await;
            }
        })
    }

    pub(crate) fn init() -> (Self, Task<MinusGamesGuiMessage>) {
        let theme_string = get_gui_config().theme.as_str();

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

        (MinusGamesGui::default(), Self::start_async_init())
    }

    pub(crate) fn start_async_init() -> Task<MinusGamesGuiMessage> {
        Task::perform(
            MinusGamesGui::async_init(),
            MinusGamesGuiMessage::InitComplete,
        )
    }

    // pub(crate) async fn async_init() -> Arc<RwLock<tokio::sync::mpsc::Receiver<MinusGamesClientEvents>>> {
    pub(crate) async fn async_init() {
        info!("Async Init!");
    }

    pub(crate) fn load() -> Task<MinusGamesGuiMessage> {
        Task::perform(MinusGamesGui::create_cards(), MinusGamesGuiMessage::Created)
    }

    pub(crate) async fn create_cards() -> Vec<GameCard> {
        debug!("Create Cards");
        let installed_games = get_installed_games();
        let mut server_games_with_date = get_client()
            .get_games_with_date_list()
            .await
            .unwrap_or_default();
        server_games_with_date.sort_unstable_by(|l, r| r.date.cmp(&l.date));
        let mut server_games: Vec<String> = Vec::with_capacity(server_games_with_date.len());

        for game in server_games_with_date {
            server_games.push(game.name);
        }

        let mut rtn = Vec::new();
        for game in &installed_games {
            let content = if server_games.contains(game) {
                "Installed - On Server"
            } else {
                "Installed"
            };
            let game_card = GameCard::new(
                game.to_string(),
                content.into(),
                true,
                rtn.len(),
                server_games.contains(game),
            );
            rtn.push(game_card);
        }

        for game in &server_games {
            if installed_games.contains(game) {
                continue;
            }
            let game_card =
                GameCard::new(game.to_string(), "On Server".into(), false, rtn.len(), true);
            rtn.push(game_card);
        }

        rtn
    }

    pub(crate) fn update(&mut self, message: MinusGamesGuiMessage) -> Task<MinusGamesGuiMessage> {
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
                return Task::batch([
                    text_input::focus(FILTER_ID),
                    Task::done(MinusGamesGuiMessage::FilterChanged(self.filter.clone())),
                ]);
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
                        delete_game(&game, true);
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
            MinusGamesGuiMessage::OpenGameModal(game, is_installed) => {
                self.model = Some((game, is_installed));
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
                self.current_game = None;
                self.current_game_name = None;
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
                return Task::perform(Self::close(), MinusGamesGuiMessage::CloseApplication);
            }
            MinusGamesGuiMessage::CloseApplication(_) => {
                info!("Client event listener closed!");
                CLOSING.store(true, Relaxed);
                return window::get_latest().and_then(window::close);
            }
            MinusGamesGuiMessage::Event(event) => {
                return handle_keyboard_events(self, event);
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
                };
            }
            MinusGamesGuiMessage::BackFromSettings(save) => {
                if save {
                    save_new_settings(self.settings.as_ref());
                    override_config(self.settings.as_ref());
                    if let Some(settings) = self.settings.take() {
                        self.theme = settings.theme;
                    }
                    reset_client();
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
                if change.is_empty() {
                    self.filter = change;
                    self.highlight_map = self.game_cards.iter().map(|g| g.position).collect();
                } else if change != self.filter {
                    self.filter = change;
                    let used_filter = self.filter.trim().to_lowercase();
                    self.highlight_map.clear();
                    for game_card in self.game_cards.iter() {
                        if game_card.game.to_lowercase().contains(&used_filter) {
                            self.highlight_map.push(game_card.position);
                        }
                    }

                    self.current_highlight_position = 0;
                } else {
                    info!("Nothing changed");
                }
            }
            MinusGamesGuiMessage::UpdateAllGames => {
                self.state = MinusGamesState::Loading;
                return Task::perform(
                    async { repair_all_games().await },
                    MinusGamesGuiMessage::FinishedRepairing,
                );
            }
            MinusGamesGuiMessage::RescanGameFolder => {
                self.state = MinusGamesState::Loading;
                return Task::perform(
                    async { scan_for_games() },
                    MinusGamesGuiMessage::FinishedRepairing,
                );
            }
            MinusGamesGuiMessage::StopDownload => {
                info!("Stop Download");
                STOP_DOWNLOAD.store(true, Relaxed);
            }
            MinusGamesGuiMessage::KillCurrentGame => {
                kill_current_running_game();
            }
            MinusGamesGuiMessage::EnterMouseArea(position) => {
                if self.last_input_mouse && self.model.is_none() {
                    if let Some((idx, _)) = self
                        .highlight_map
                        .iter()
                        .enumerate()
                        .find(|&(ref _idx, &i)| i == position)
                    {
                        self.current_highlight_position = idx;
                    }
                }
            }
            MinusGamesGuiMessage::CurrentPositionUp(up) => {
                if self.state == MinusGamesState::Ready
                    && !self.highlight_map.is_empty()
                    && self.current_highlight_position != 0
                {
                    self.last_input_mouse = false;
                    self.current_highlight_position = cmp::min(
                        self.current_highlight_position.saturating_sub(up),
                        self.highlight_map.len() - 1,
                    );
                    self.block_highlighting = true;
                    self.went_up = true;
                    return Task::done(MinusGamesGuiMessage::ScrollUp(up));
                }
            }
            MinusGamesGuiMessage::CurrentPositionDown(down) => {
                if self.state == MinusGamesState::Ready
                    && !self.highlight_map.is_empty()
                    && self.current_highlight_position != self.highlight_map.len() - 1
                {
                    self.last_input_mouse = false;
                    self.current_highlight_position = cmp::min(
                        self.current_highlight_position
                            .checked_add(down)
                            .unwrap_or(self.highlight_map.len() - 1),
                        self.highlight_map.len() - 1,
                    );
                    self.block_highlighting = true;
                    self.went_up = false;
                    return Task::done(MinusGamesGuiMessage::ScrollDown(down));
                }
            }
            MinusGamesGuiMessage::StartCurrentPosition => {
                if self.state == MinusGamesState::Ready {
                    let position = self.current_highlight_position;
                    if let Some(game_card_position) = self.highlight_map.get(position) {
                        if let Some(game_card) = self.game_cards.get(*game_card_position) {
                            return Task::done(MinusGamesGuiMessage::Play(game_card.game.clone()));
                        }
                    }
                }
            }
            MinusGamesGuiMessage::ScrollDown(step) => {
                let screen_tiles = self.size.height as u16 / GAME_CARD_ROW_HEIGHT;

                let hidden_tiles = self.scroll_offset.y as u16 / GAME_CARD_ROW_HEIGHT;

                let start_tile = hidden_tiles;
                let end_tile = start_tile + screen_tiles;

                self.block_highlighting = false;
                if !self.went_up && end_tile - 6 < self.current_highlight_position as u16 {
                    return scrollable::scroll_by(
                        SCROLLABLE_ID.clone(),
                        AbsoluteOffset {
                            x: 0.0,
                            y: step as f32 * GAME_CARD_ROW_HEIGHT as f32,
                        },
                    );
                }
            }
            MinusGamesGuiMessage::ScrollUp(step) => {
                let hidden_tiles = self.scroll_offset.y as u16 / GAME_CARD_ROW_HEIGHT;

                let start_tile = hidden_tiles;

                self.block_highlighting = false;
                if self.went_up && start_tile + 1 > self.current_highlight_position as u16 {
                    return scrollable::scroll_by(
                        SCROLLABLE_ID.clone(),
                        AbsoluteOffset {
                            x: 0.0,
                            y: -(step as f32 * GAME_CARD_ROW_HEIGHT as f32),
                        },
                    );
                }
            }
            MinusGamesGuiMessage::Scrolled(viewport) => {
                self.scroll_offset = viewport.absolute_offset();
            }
        };
        Task::none()
    }

    async fn close() {
        send_event(MinusGamesClientEvents::Close).await;
    }

    pub(crate) fn view(&self) -> Element<MinusGamesGuiMessage> {
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
            .id(SCROLLABLE_ID.clone())
            .on_scroll(MinusGamesGuiMessage::Scrolled)
            .width(Fill)
            .height(Fill);

        match &self.model {
            None => content.into(),
            Some((game, is_on_server)) => stack!(
                content,
                create_modal(game, *is_on_server, self.size.width).into()
            )
            .into(),
        }
    }

    fn create_ready_view(&self) -> Column<MinusGamesGuiMessage> {
        if self.game_cards.is_empty() {
            return column![
                row![
                    horizontal_space().width(Fill),
                    text("No Games found...").size(50),
                    horizontal_space().width(Fill),
                ],
                row![
                    horizontal_space().width(Fill),
                    button("Reload")
                        .on_press(MinusGamesGuiMessage::Reload)
                        .padding(TOP_BUTTON),
                    horizontal_space().width(MARGIN_DEFAULT),
                    button("Settings")
                        .on_press(MinusGamesGuiMessage::Settings)
                        .padding(TOP_BUTTON),
                    horizontal_space().width(MARGIN_DEFAULT),
                    button("Quit")
                        .on_press(MinusGamesGuiMessage::CloseApplication(()))
                        .padding(TOP_BUTTON),
                    horizontal_space().width(Fill),
                ]
            ];
        }
        let mut rtn = Column::with_capacity(self.game_cards.len() + 1);
        rtn = rtn.push(
            row![
                column![
                    text("Games").size(TEXT),
                    text_input("Filter", &self.filter)
                        .id(FILTER_ID)
                        .on_input(MinusGamesGuiMessage::FilterChanged)
                        .on_submit(MinusGamesGuiMessage::StartCurrentPosition)
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
        rtn = rtn.push(vertical_space().height(HALF_MARGIN_DEFAULT));
        let position = self.current_highlight_position;

        for (idx, game_card_position) in self.highlight_map.iter().enumerate() {
            if let Some(game_card) = self.game_cards.get(*game_card_position) {
                if idx == position && !self.block_highlighting {
                    rtn = rtn.push(AlwaysHighlighter::new(
                        row![game_card.view()].spacing(SPACING_DEFAULT).into(),
                    ));
                } else {
                    rtn = rtn.push(row![game_card.view()].spacing(SPACING_DEFAULT));
                }
            }
        }

        rtn
    }

    pub(crate) fn get_theme(&self) -> Theme {
        self.theme.clone()
    }

    // fn handle_event(&mut self, event: Event) -> Task<MinusGamesGuiMessage> {
    //     match event {
    //         Event::Keyboard(KeyPressed {
    //             key: Key::Character(character),
    //             modifiers,
    //             ..
    //         }) => {
    //             if character.as_str() == "f" && modifiers.control() {
    //                 return Task::batch([
    //                     snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START),
    //                     text_input::focus(FILTER_ID),
    //                 ]);
    //             }
    //         }
    //         Event::Keyboard(KeyPressed {
    //             key: Key::Named(named),
    //             modifiers,
    //             ..
    //         }) => match named {
    //             key::Named::Tab => {
    //                 return if modifiers.shift() {
    //                     widget::focus_previous()
    //                 } else {
    //                     widget::focus_next()
    //                 };
    //             }
    //             key::Named::ArrowUp => {
    //                 return Task::done(MinusGamesGuiMessage::CurrentPositionUp);
    //             }
    //             key::Named::ArrowDown => {
    //                 return Task::done(MinusGamesGuiMessage::CurrentPositionDown);
    //             }
    //             key::Named::Enter => {
    //                 return Task::done(MinusGamesGuiMessage::StartCurrentPosition);
    //             }
    //             key::Named::Home => {
    //                 return snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START);
    //             }
    //             key::Named::End => {
    //                 return snap_to(SCROLLABLE_ID.clone(), RelativeOffset::END);
    //             }
    //             key::Named::PageUp => {
    //                 return scroll_by(
    //                     SCROLLABLE_ID.clone(),
    //                     AbsoluteOffset {
    //                         x: 0.0,
    //                         y: -((GAME_CARD_ROW_HEIGHT * 3) as f32),
    //                     },
    //                 );
    //             }
    //             key::Named::PageDown => {
    //                 return scroll_by(
    //                     SCROLLABLE_ID.clone(),
    //                     AbsoluteOffset {
    //                         x: 0.0,
    //                         y: (GAME_CARD_ROW_HEIGHT * 3) as f32,
    //                     },
    //                 );
    //             }
    //             _ => {}
    //         },
    //         Event::Mouse(mouse::Event::ButtonPressed(Button::Back)) => {
    //             return Task::done(MinusGamesGuiMessage::BackFromSettings(false));
    //         }
    //         Event::Mouse(mouse::Event::CursorMoved { .. }) => {
    //             self.last_input_mouse = true;
    //         }
    //         Event::Window(window::Event::CloseRequested) => {
    //             info!("Close Application!");
    //             return Task::perform(Self::close(), MinusGamesGuiMessage::CloseApplication);
    //         }
    //         Event::Window(window::Event::Resized(size)) => {
    //             self.size = size;
    //         }
    //         Event::Window(window::Event::Closed) => {
    //             CLOSING.store(true, Relaxed);
    //         }
    //         _ => {}
    //     }
    //
    //     Task::none()
    // }
}
