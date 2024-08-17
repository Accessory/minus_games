use crate::actions::delete::delete_game;
use crate::actions::download::download;
#[cfg(target_family = "unix")]
use crate::actions::menu::select_game_to_play;
use crate::actions::menu::{
    select_download, select_game, select_game_to_delete, select_repair, start_menu,
};
use crate::actions::other::{list, list_json};
use crate::actions::repair::repair_game;
use crate::actions::run::{run_game, sync_run_game};
use crate::actions::scan::scan_for_games;
use crate::actions::sync::{download_syncs, sync_infos_for_all_games, upload_syncs};
use crate::configuration::ClientActions;
use crate::runtime::{CONFIG, OFFLINE};
use actions::run::run_game_synced;
use actions::sync::download_sync_for_game;
use std::ops::Deref;
use std::sync::atomic::Ordering;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

mod actions;
mod configuration;
mod download_manager;
mod minus_games_client;
mod runtime;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // Configuration

    if let Some(config_dir) = dirs::config_local_dir() {
        let config_path = config_dir.join("minus_games_client").join("config");
        if config_path.exists() {
            dotenvy::from_filename_override(config_path).ok();
        }
    }

    if CONFIG.action != ClientActions::ListJson {
        println!("Config:");
        println!("{}", CONFIG.deref());
    }

    // Logging
    let filter = if CONFIG.verbose {
        EnvFilter::default()
            .add_directive(LevelFilter::TRACE.into())
            .add_directive("minus_games_client=debug".parse().unwrap())
    } else {
        EnvFilter::default()
            .add_directive(LevelFilter::INFO.into())
            .add_directive("minus_games_client=debug".parse().unwrap())
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Offline
    OFFLINE.store(CONFIG.offline, Ordering::Relaxed);

    // Main
    match &CONFIG.action {
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
        ClientActions::RunGame { game } => run_game(game),
        ClientActions::RunGameSynced { game } => run_game_synced(game).await,
        ClientActions::SyncRunGame { game } => sync_run_game(game).await,
        ClientActions::SelectGame => select_game().await,
        ClientActions::DeleteGame { game, purge } => delete_game(game, purge.unwrap_or(true)),
        ClientActions::SelectDeleteGame { purge } => select_game_to_delete(purge.unwrap_or(true)),
        ClientActions::Menu => start_menu().await,
        ClientActions::Repair { game } => repair_game(game).await,
        ClientActions::SelectRepair => select_repair().await,
        ClientActions::DownloadSyncs => download_syncs().await,
        ClientActions::DownloadSync { game } => download_sync_for_game(game).await,
        ClientActions::UploadSyncs => upload_syncs().await,
        ClientActions::ScanForGames => scan_for_games(),
        #[cfg(target_family = "unix")]
        ClientActions::SelectGameToPlay => select_game_to_play().await,
    }
}
