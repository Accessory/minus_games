use crate::actions::sync::force_sync_all_game_files;
use crate::runtime::{MinusGamesClientEvents, STOP_DOWNLOAD, get_installed_games, send_event};
use std::sync::atomic::Ordering::Relaxed;

pub async fn repair_game(game: &str) {
    STOP_DOWNLOAD.store(false, Relaxed);
    send_event(MinusGamesClientEvents::CurrentGame(game.to_string())).await;
    force_sync_all_game_files(game).await;
    send_event(MinusGamesClientEvents::FinishedSyncFileInfos).await;
}

pub async fn repair_all_games() {
    STOP_DOWNLOAD.store(false, Relaxed);
    let games = get_installed_games();
    for game in games {
        send_event(MinusGamesClientEvents::CurrentGame(game.to_string())).await;
        force_sync_all_game_files(&game).await;
    }
    send_event(MinusGamesClientEvents::FinishedSyncFileInfos).await;
}
