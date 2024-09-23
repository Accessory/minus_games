use indicatif::ProgressBar;
use minus_games_client::actions::delete::delete_game;
use minus_games_client::actions::download::download;
#[cfg(target_family = "unix")]
use minus_games_client::actions::menu::select_game_to_play;
use minus_games_client::actions::menu::{
    select_download, select_game, select_game_to_delete, select_repair, start_menu,
};
use minus_games_client::actions::other::{list, list_json};
use minus_games_client::actions::repair::repair_game;
use minus_games_client::actions::run::run_game_synced;
use minus_games_client::actions::run::{run_game, sync_run_game};
use minus_games_client::actions::scan::scan_for_games;
use minus_games_client::actions::sync::download_sync_for_game;
use minus_games_client::actions::sync::{download_syncs, sync_infos_for_all_games, upload_syncs};
use minus_games_client::configuration::ClientActions;
use minus_games_client::runtime::{
    get_config, send_event, set_sender, MinusGamesClientEvents, OFFLINE,
};
use std::sync::atomic::Ordering;
use tracing::{debug, info, warn};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Configuration
    if let Some(config_dir) = dirs::config_local_dir() {
        let config_path = config_dir.join("minus_games_client").join("config");
        if config_path.exists() {
            dotenvy::from_filename_override(config_path).ok();
        }
    }
    dotenvy::dotenv().ok();

    let action = get_config()
        .action
        .as_ref()
        .unwrap_or(&ClientActions::Menu)
        .clone();

    if action != ClientActions::ListJson {
        println!("Config:");
        println!("{}", get_config());
    }

    // Logging
    let filter = if get_config().verbose {
        EnvFilter::default()
            .add_directive(LevelFilter::TRACE.into())
            .add_directive("minus_games_client=debug".parse().unwrap())
    } else {
        EnvFilter::default().add_directive(LevelFilter::INFO.into())
        // .add_directive("minus_games_client=debug".parse().unwrap())
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

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
