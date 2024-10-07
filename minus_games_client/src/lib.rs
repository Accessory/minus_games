use crate::actions::delete::delete_game;
use crate::actions::download::download;
#[cfg(target_family = "unix")]
use crate::actions::menu::select_game_to_play;
use crate::actions::menu::{
    select_download, select_game, select_game_to_delete, select_repair, start_menu,
};
use crate::actions::other::{list, list_json};
use crate::actions::repair::repair_game;
use crate::actions::run::{run_game, run_game_synced, sync_run_game};
use crate::actions::scan::scan_for_games;
use crate::actions::sync::{
    download_sync_for_game, download_syncs, sync_infos_for_all_games, upload_syncs,
};
use crate::configuration::ClientActions;
use crate::runtime::{get_config, send_event, set_sender, MinusGamesClientEvents, OFFLINE};
use indicatif::ProgressBar;
use std::sync::atomic::Ordering;
use tracing::{debug, info, warn};

pub mod actions;
pub mod configuration;
pub mod download_manager;
pub mod minus_games_client;
pub mod runtime;
pub mod utils;

pub async fn run_cli() {
    let action = get_config()
        .action
        .as_ref()
        .unwrap_or(&ClientActions::Menu)
        .clone();

    // Offline
    OFFLINE.store(get_config().offline, Ordering::Relaxed);

    // Init EventManager
    let (sender, mut receiver) =
        tokio::sync::mpsc::channel(std::thread::available_parallelism().unwrap().get());
    set_sender(sender).await;
    let event_handle = tokio::task::spawn(async move {
        let mut bar_option: Option<ProgressBar> = None;
        while let Some(event) = receiver.recv().await {
            match event {
                MinusGamesClientEvents::StartDownloadingFiles(files_count) => {
                    let _ = bar_option.insert(ProgressBar::new(files_count as u64));
                }
                MinusGamesClientEvents::FinishedDownloadingFile => {
                    if let Some(bar) = &bar_option {
                        bar.inc(1);
                    }
                }
                MinusGamesClientEvents::FinishedDownloadingFiles => {
                    if let Some(bar) = &bar_option {
                        bar.finish();
                    }
                    bar_option = None;
                }
                MinusGamesClientEvents::LogInfoMessage(msg) => {
                    info!("{msg}");
                }
                MinusGamesClientEvents::LogInfoStaticMessage(msg) => {
                    info!("{msg}");
                }
                MinusGamesClientEvents::Close => {
                    return;
                }
                _ => {
                    debug!("Caught event: {}", event);
                }
            }
        }
    });

    // Main
    match action {
        ClientActions::List => {
            list().await;
        }
        ClientActions::ListJson => {
            list_json().await;
        }
        ClientActions::Download(game) => {
            download(&game.game).await;
        }
        ClientActions::Sync => sync_infos_for_all_games().await,
        ClientActions::SelectDownload => select_download().await,
        ClientActions::RunGame { game } => run_game(&game).await,
        ClientActions::RunGameSynced { game } => run_game_synced(&game).await,
        ClientActions::SyncRunGame { game } => sync_run_game(&game).await,
        ClientActions::SelectGame => select_game().await,
        ClientActions::DeleteGame { game, purge } => delete_game(&game, purge.unwrap_or(true)),
        ClientActions::SelectDeleteGame { purge } => select_game_to_delete(purge.unwrap_or(true)),
        ClientActions::Menu => start_menu().await,
        ClientActions::Repair { game } => repair_game(&game).await,
        ClientActions::SelectRepair => select_repair().await,
        ClientActions::DownloadSyncs => download_syncs().await,
        ClientActions::DownloadSync { game } => download_sync_for_game(&game).await,
        ClientActions::UploadSyncs => upload_syncs().await,
        ClientActions::ScanForGames => scan_for_games(),
        #[cfg(target_family = "unix")]
        ClientActions::SelectGameToPlay => select_game_to_play().await,
        ClientActions::Gui => {
            warn!("Gui mode is not supported by the client");
        }
    }

    // Cleanup
    send_event(MinusGamesClientEvents::Close).await;
    event_handle.await.unwrap();
}
