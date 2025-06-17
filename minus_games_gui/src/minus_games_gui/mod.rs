use crate::minus_games_gui::game_card::GameCard;
use crate::minus_games_gui::handlers::gamepad_input_handler::gamepad_input_handler;
use crate::minus_games_gui::handlers::keyboard_event_handler::handle_system_events;
use crate::minus_games_gui::handlers::lazy_image_download_handler::lazy_image_download_handler;
use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::messages::modal_callback::ModalCallback;
use crate::minus_games_gui::minus_games_settings::MinusGamesSettings;
use crate::minus_games_gui::settings::{
    handle_change_event, override_gui_config, save_new_settings,
};
use crate::minus_games_gui::style_constants::{
    GAME_CARD_ROW_HEIGHT, HALF_MARGIN_DEFAULT, MARGIN_DEFAULT, SPACING_DEFAULT, TEXT,
};
use crate::minus_games_gui::views::buttons_helper::{create_config_button, create_quit_button};
use crate::minus_games_gui::views::game_info_modal::create_modal;
use crate::minus_games_gui::views::{downloading, gaming, loading, ready, settings_view};
use crate::minus_games_gui::widgets::always_highlighter::AlwaysHighlighter;
use crate::runtime::{CLOSING, SCROLLABLE_ID, get_gui_config, get_mut_gui_config};
use iced::Bottom;
use iced::futures::channel::mpsc;
use iced::futures::channel::mpsc::Sender;
use iced::futures::{SinkExt, Stream};
use iced::widget::scrollable::Anchor::Start;
use iced::widget::scrollable::{AbsoluteOffset, Direction, RelativeOffset, Scrollbar};
use iced::widget::{
    Button, Column, button, column, horizontal_space, row, scrollable, stack, text, text_input,
    vertical_space,
};
use iced::{Center, Element, Fill, Length, Size, Subscription, Task, Theme, event, stream, window};
use minus_games_client::actions::delete::delete_game;
use minus_games_client::actions::other::move_additions_header_to_tmp;
use minus_games_client::actions::repair::{repair_all_games, repair_game};
use minus_games_client::actions::run::run_game_synced;
use minus_games_client::actions::scan::scan_for_games;
use minus_games_client::runtime::{
    MinusGamesClientEvents, STOP_DOWNLOAD, get_client, get_config, get_installed_games,
    kill_current_running_game, reset_client, send_event, set_sender,
};
use minus_games_models::game_infos::{GameInfos, MinimalGameInfos};
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
    pub scale: Option<f64>,
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
    pub lazy_image_downloader_sender: Option<Sender<(String, bool, usize)>>,
}

const FILTER_ID: &str = "FILTER_ID";

impl MinusGamesGui {
    pub(crate) fn title(&self) -> String {
        match self.state {
            MinusGamesState::Loading => "Loading - Minus Games".to_string(),
            MinusGamesState::Ready => "Minus Games".to_string(),
            MinusGamesState::Downloading => match self.current_game.as_ref() {
                None => "Downloading - Minus Games".to_string(),
                Some(value) => {
                    format!("Downloading - {} - Minus Games", value.folder_name)
                }
            },
            MinusGamesState::Gaming => match self.current_game.as_ref() {
                None => "Gaming - Minus Games".to_string(),
                Some(value) => {
                    format!("Gaming - {} - Minus Games", value.folder_name)
                }
            },
            MinusGamesState::Settings => "Settings - Minus Games".to_string(),
        }
    }
    pub(crate) fn batch_subscription(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::batch([
            self.event_subscription(),
            self.on_close(),
            self.start_fullscreen(),
            self.gamepad_input(),
            self.lazy_image_download(),
        ])
    }

    pub(crate) fn start_fullscreen(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::run(Self::set_start_fullscreen)
    }

    pub(crate) fn set_start_fullscreen() -> impl Stream<Item = MinusGamesGuiMessage> {
        stream::channel(
            1,
            |mut output: mpsc::Sender<MinusGamesGuiMessage>| async move {
                if get_gui_config().fullscreen {
                    let _ = output.send(MinusGamesGuiMessage::Fullscreen).await;
                }
            },
        )
    }

    pub(crate) fn lazy_image_download(&self) -> Subscription<MinusGamesGuiMessage> {
        Subscription::run(lazy_image_download_handler)
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
        stream::channel(
            24,
            |mut output: mpsc::Sender<MinusGamesGuiMessage>| async move {
                let (sender, mut receiver) =
                    tokio::sync::mpsc::channel(std::thread::available_parallelism().unwrap().get());
                set_sender(sender).await;
                while let Some(event) = receiver.recv().await {
                    let _ = output.send(event.into()).await;
                }
            },
        )
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
            MinusGamesGuiMessage::InitWindow,
        )
    }

    // pub(crate) async fn async_init() -> Arc<RwLock<tokio::sync::mpsc::Receiver<MinusGamesClientEvents>>> {
    pub(crate) async fn async_init() {
        info!("Async Init!");
    }

    pub(crate) fn load() -> Task<MinusGamesGuiMessage> {
        Task::perform(MinusGamesGui::create_cards(), MinusGamesGuiMessage::Created)
    }

    fn apply_filter(&mut self, change: String, force: bool) {
        if change.is_empty() {
            self.filter = change;
            self.highlight_map = self.game_cards.iter().map(|g| g.position).collect();
        } else if force || change != self.filter {
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

    pub(crate) async fn create_cards() -> Vec<GameCard> {
        debug!("Create Cards");
        let mut installed_games = get_installed_games();

        installed_games.sort_by_key(|game| get_config().get_game_last_action_date(game));
        installed_games.reverse();

        let mut minimal_game_infos = get_client()
            .get_games_with_minimal_game_infos()
            .await
            .unwrap_or_default();
        minimal_game_infos.sort_unstable_by(|l, r| r.date.cmp(&l.date));
        let mut server_games: Vec<String> = Vec::with_capacity(minimal_game_infos.len());
        let mut has_header_list: Vec<bool> = Vec::with_capacity(minimal_game_infos.len());
        let mut info_list = Vec::with_capacity(minimal_game_infos.len());

        for game in minimal_game_infos {
            server_games.push(game.name);
            has_header_list.push(game.header);
            info_list.push(game.minimal_game_infos);
        }

        let mut rtn = Vec::new();
        for game in &installed_games {
            let info: MinimalGameInfos = match server_games
                .iter()
                .enumerate()
                .find(|(_idx, game_name)| game_name == &game)
            {
                None => match get_config().get_game_infos(game) {
                    None => continue,
                    Some(game_infos) => game_infos.into(),
                },
                Some((idx, _)) => info_list.get(idx).unwrap().clone(),
            };

            let image_option = get_config()
                .get_header_option(game)
                .map(iced::widget::image::Handle::from_path);

            let has_header = image_option.is_some();

            let game_card = GameCard::new(
                game.to_string(),
                // content.into(),
                true,
                rtn.len(),
                image_option,
                server_games.contains(game),
                has_header,
                info,
            );
            rtn.push(game_card);
        }

        for (idx, game) in server_games.iter().enumerate() {
            if installed_games.contains(game) {
                continue;
            }

            let image_option = get_config()
                .get_header_option(game)
                .map(iced::widget::image::Handle::from_path);

            let has_header = image_option.is_some() || has_header_list[idx];

            let game_card = GameCard::new(
                game.to_string(),
                // "\n".into(),
                false,
                rtn.len(),
                image_option,
                true,
                has_header,
                info_list.get(idx).unwrap().clone(),
            );
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

                let mut to_send = Vec::new();

                for game_card in self.game_cards.iter() {
                    if game_card.image.is_none() && game_card.is_on_server && game_card.has_header {
                        to_send.push((
                            game_card.game.clone(),
                            game_card.is_installed,
                            game_card.position,
                        ));
                    }
                }

                let mut inner_sender = self.lazy_image_downloader_sender.clone();
                let image_task = Task::perform(
                    async move {
                        if let Some(sender) = inner_sender.as_mut() {
                            for send in to_send {
                                sender.send(send).await.ok();
                            }
                        }
                        debug!("Finished sending game card image paths");
                    },
                    MinusGamesGuiMessage::FinishedProcessingImages,
                );

                self.apply_filter(self.filter.clone(), true);

                return Task::batch([
                    text_input::focus(FILTER_ID),
                    // Task::done(MinusGamesGuiMessage::FilterChanged(self.filter.clone(),true)),
                    image_task,
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
                        move_additions_header_to_tmp(&game);
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
                self.current_highlight_position = 0;
                return Task::batch([
                    Task::done(MinusGamesGuiMessage::ScrollToTop),
                    MinusGamesGui::load(),
                ]);
            } // _ => Task::none(),
            MinusGamesGuiMessage::Init => {}
            MinusGamesGuiMessage::SetScale(scale) => {
                self.scale = Some(scale);
                get_mut_gui_config().scale = self.scale;
            }
            MinusGamesGuiMessage::InitWindow(_) => {
                return if let Some(scale) = get_gui_config().scale {
                    Task::batch([
                        Task::done(MinusGamesGuiMessage::SetScale(scale)),
                        Task::done(MinusGamesGuiMessage::InitComplete(())),
                    ])
                } else {
                    window::get_latest().and_then(|window_id| {
                        window::get_scale_factor(window_id).then(|f| {
                            // println!("Scale Factor: {f}");
                            Task::batch([
                                Task::done(MinusGamesGuiMessage::SetScale(f as f64)),
                                Task::done(MinusGamesGuiMessage::InitComplete(())),
                            ])
                        })
                    })
                };
            }
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
                if self.current_game.is_none()
                    && let Some(game) = &self.current_game_name
                {
                    self.current_game = get_config().get_game_infos(game);
                }

                self.state = MinusGamesState::Gaming;
            }
            MinusGamesGuiMessage::CloseGame(_) => {
                self.current_game = None;
                self.current_game_name = None;
                self.current_highlight_position = 0;
                self.state = MinusGamesState::Loading;
                return Task::batch([Self::load(), Task::done(MinusGamesGuiMessage::ScrollToTop)]);
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
                return handle_system_events(self, event);
            }
            MinusGamesGuiMessage::Fullscreen => {
                return window::get_latest()
                    .and_then(move |window| window::set_mode(window, window::Mode::Fullscreen));
            }
            MinusGamesGuiMessage::GotoSettings => {
                self.settings = Some(MinusGamesSettings::from_config_with_theme(
                    get_config(),
                    get_gui_config(),
                    self.get_theme(),
                ));
                self.state = MinusGamesState::Settings;
            }
            MinusGamesGuiMessage::ApplyScreenSettings => {
                return if get_gui_config().fullscreen {
                    window::get_latest()
                        .and_then(move |window| window::set_mode(window, window::Mode::Fullscreen))
                } else {
                    window::get_latest()
                        .and_then(move |window| window::set_mode(window, window::Mode::Windowed))
                };
            }
            MinusGamesGuiMessage::BackAction => match self.state {
                MinusGamesState::Ready => {
                    return Task::done(MinusGamesGuiMessage::CloseApplication(()));
                }
                MinusGamesState::Settings => {
                    return Task::done(MinusGamesGuiMessage::BackFromSettings(false));
                }
                _ => {}
            },
            MinusGamesGuiMessage::StartAction => {
                if self.state == MinusGamesState::Ready {
                    return Task::done(MinusGamesGuiMessage::GotoSettings);
                }
            }
            MinusGamesGuiMessage::ReloadAction => {
                if self.state == MinusGamesState::Ready {
                    return Task::done(MinusGamesGuiMessage::Reload);
                }
            }
            MinusGamesGuiMessage::BackFromSettings(save) => {
                if save {
                    save_new_settings(self.settings.as_ref());
                    override_config(self.settings.as_ref());
                    override_gui_config(self.settings.as_ref());
                    if let Some(settings) = self.settings.take() {
                        self.theme = settings.theme;
                        self.scale = Some(settings.scale);
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
                if let Some(settings) = self.settings.as_ref()
                    && self.theme != settings.theme
                {
                    self.theme = settings.theme.clone();
                }
                // if !self.scale.is_some_and(|v| v == settings.scale) {
                //     self.scale = Some(settings.scale);
                // }
            }
            MinusGamesGuiMessage::FilterChanged(change) => {
                self.apply_filter(change, false);
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
                if self.last_input_mouse
                    && self.model.is_none()
                    && let Some((idx, _)) = self
                        .highlight_map
                        .iter()
                        .enumerate()
                        .find(|&(ref _idx, &i)| i == position)
                {
                    self.current_highlight_position = idx;
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
                    if let Some(game_card_position) = self.highlight_map.get(position)
                        && let Some(game_card) = self.game_cards.get(*game_card_position)
                    {
                        return Task::done(MinusGamesGuiMessage::Play(game_card.game.clone()));
                    }
                }
            }
            MinusGamesGuiMessage::ScrollDown(step) => {
                let screen_height = self.get_screen_height();

                let ok_area = screen_height * 0.80;
                let top = 2.0 * GAME_CARD_ROW_HEIGHT as f32
                    + self.current_highlight_position as f32 * GAME_CARD_ROW_HEIGHT as f32
                    - self.scroll_offset.y;

                let bottom = top + GAME_CARD_ROW_HEIGHT as f32;

                // trace!("ok_area: {}, top: {}, bottom: {}, screen_height: {}", ok_area, top, bottom, screen_height);

                self.block_highlighting = false;
                if !self.went_up && ok_area < bottom {
                    return scrollable::scroll_by(
                        SCROLLABLE_ID.clone(),
                        AbsoluteOffset {
                            x: 0.0,
                            y: step as f32 * GAME_CARD_ROW_HEIGHT as f32,
                        },
                    );
                }
            }
            MinusGamesGuiMessage::ScrollToTop => {
                return scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START);
            }
            MinusGamesGuiMessage::ScrollUp(step) => {
                self.block_highlighting = false;
                if self.current_highlight_position == 0 {
                    return scrollable::snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START);
                }

                let screen_height = self.get_screen_height();

                let not_ok_area = screen_height * 0.17;
                let top = 2.0 * GAME_CARD_ROW_HEIGHT as f32
                    + self.current_highlight_position as f32 * GAME_CARD_ROW_HEIGHT as f32
                    - self.scroll_offset.y;

                // trace!("not_ok_area: {}, top: {}, screen_height: {}", not_ok_area, top, screen_height);

                if self.went_up && not_ok_area > top {
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
            MinusGamesGuiMessage::LazyImageDownloaderReady(sender) => {
                self.lazy_image_downloader_sender = Some(sender);
            }
            MinusGamesGuiMessage::LazyImageUpdateCard(card_id, handle) => {
                if let Some(card) = self.game_cards.get_mut(card_id) {
                    card.image = Some(handle);
                }
                return Task::done(MinusGamesGuiMessage::FinishedProcessingImages(()));
            }
            MinusGamesGuiMessage::FinishedProcessingImages(_) => {
                debug!("Finished Processing Images. There are now lazy loaded");
            }
        };
        Task::none()
    }

    async fn close() {
        send_event(MinusGamesClientEvents::Close).await;
    }

    pub(crate) fn view(&self) -> Element<'_, MinusGamesGuiMessage> {
        let to_display = match self.state {
            MinusGamesState::Loading => loading::view(self.size.height),
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
            None => stack!(content).into(),
            Some((game, is_on_server)) => stack!(
                content,
                create_modal(game, *is_on_server, self.size.width).into()
            )
            .into(),
        }
    }

    fn create_ready_view(&self) -> Column<'_, MinusGamesGuiMessage> {
        if self.game_cards.is_empty() {
            return column![
                row![
                    horizontal_space().width(Fill),
                    text("No Games found...").size(50),
                    horizontal_space().width(Fill),
                ],
                row![
                    horizontal_space().width(Fill),
                    create_reload_button(),
                    horizontal_space().width(MARGIN_DEFAULT),
                    create_settings_button(),
                    horizontal_space().width(MARGIN_DEFAULT),
                    create_quit_button(),
                    horizontal_space().width(Fill),
                ]
            ];
        }
        let mut rtn = Column::with_capacity(self.game_cards.len() + 1);
        rtn = rtn.push(
            row![column![
                row![
                    text("Games").size(TEXT),
                    // text(format!(
                    //     "X {}, Y {}",
                    //     self.size.width,
                    //     self.get_screen_height()
                    // )),
                    horizontal_space().width(Fill),
                    create_reload_button(),
                    horizontal_space().width(HALF_MARGIN_DEFAULT),
                    create_settings_button(),
                    horizontal_space().width(MARGIN_DEFAULT),
                    create_quit_button(),
                ]
                .align_y(Bottom),
                vertical_space().height(HALF_MARGIN_DEFAULT),
                row![
                        text_input("Filter", &self.filter)
                            .id(FILTER_ID)
                            .on_input(MinusGamesGuiMessage::FilterChanged)
                            .on_submit(MinusGamesGuiMessage::StartCurrentPosition)
                            .width(Fill),
                        button("") // Clear Filter
                            .on_press_with(|| MinusGamesGuiMessage::FilterChanged("".to_string()))
                    ]
            ],]
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

    pub(crate) fn get_screen_height(&self) -> f32 {
        // if get_gui_config().fullscreen && std::env::var("SteamDeck").is_ok_and(|v| v == "1") {
        //     *PRIMARY_SCREEN_DISPLAY_HEIGHT
        // } else {
        self.size.height / self.scale.unwrap_or(1.0) as f32
        // }
    }
}

fn create_reload_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_config_button("󰑓", MinusGamesGuiMessage::Reload)
}
fn create_settings_button<'a>() -> Button<'a, MinusGamesGuiMessage> {
    create_config_button("", MinusGamesGuiMessage::GotoSettings)
}
