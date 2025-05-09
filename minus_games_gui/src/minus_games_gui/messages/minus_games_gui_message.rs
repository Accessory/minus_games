use crate::minus_games_gui::game_card::GameCard;
use crate::minus_games_gui::messages::modal_callback::ModalCallback;
use crate::minus_games_gui::views::settings_view::SettingInput;
use iced::Event;
use iced::futures::channel::mpsc::Sender;
use iced::widget::scrollable;
use minus_games_client::runtime::MinusGamesClientEvents;
use tracing::info;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) enum MinusGamesGuiMessage {
    Init,
    // InitComplete(Arc<RwLock<tokio::sync::mpsc::Receiver<MinusGamesClientEvents>>>),
    InitWindow(()),
    SetScale(f64),
    InitComplete(()),
    Loading,
    Reload,
    Created(Vec<GameCard>),
    GotoSettings,
    StartAction,
    ReloadAction,
    BackAction,
    BackFromSettings(bool),
    ChangeSetting(SettingInput),
    Play(String),
    Delete(String),
    Repair(String),
    OpenGameModal(String, bool),
    FinishedPlay(()),
    FinishedDelete(()),
    FinishedRepairing(()),
    SetFilesToDownload(usize),
    FinishedDownloading,
    SyncFileInfosComplete,
    CurrentGame(String),
    StartGame(String),
    CloseGame(String),
    LogMessage(String),
    LogStaticMessage(&'static str),
    Fullscreen,
    ModalCallback(Option<ModalCallback>),
    ApplyScreenSettings,
    FilterChanged(String),
    Exit,
    Noop,
    CloseApplication(()),
    Event(Event),
    UpdateAllGames,
    RescanGameFolder,
    StopDownload,
    KillCurrentGame,
    EnterMouseArea(usize),
    CurrentPositionUp(usize),
    CurrentPositionDown(usize),
    StartCurrentPosition,
    ScrollToTop,
    ScrollUp(usize),
    ScrollDown(usize),
    Scrolled(scrollable::Viewport),
    LazyImageDownloaderReady(Sender<(String, bool, usize)>),
    LazyImageUpdateCard(usize, iced::widget::image::Handle),
    FinishedProcessingImages(()),
}

impl From<MinusGamesClientEvents> for MinusGamesGuiMessage {
    fn from(event: MinusGamesClientEvents) -> Self {
        match event {
            MinusGamesClientEvents::StartDownloadingFiles(files_count) => {
                MinusGamesGuiMessage::SetFilesToDownload(files_count)
            }
            MinusGamesClientEvents::StartDownloadingFile => MinusGamesGuiMessage::Noop,
            MinusGamesClientEvents::FinishedDownloadingFile => {
                MinusGamesGuiMessage::FinishedDownloading
            }
            MinusGamesClientEvents::FinishedDownloadingFiles => MinusGamesGuiMessage::Noop,
            MinusGamesClientEvents::FinishedSyncFileInfos => {
                MinusGamesGuiMessage::SyncFileInfosComplete
            }
            MinusGamesClientEvents::LogInfoMessage(msg) => MinusGamesGuiMessage::LogMessage(msg),
            MinusGamesClientEvents::StartGame(game) => MinusGamesGuiMessage::StartGame(game),
            MinusGamesClientEvents::CurrentGame(game) => MinusGamesGuiMessage::CurrentGame(game),
            MinusGamesClientEvents::LogInfoStaticMessage(msg) => {
                MinusGamesGuiMessage::LogStaticMessage(msg)
            }
            MinusGamesClientEvents::Close => MinusGamesGuiMessage::Noop,
            _ => {
                info!("Event fired: {}", &event);
                MinusGamesGuiMessage::Noop
            }
        }
    }
}
