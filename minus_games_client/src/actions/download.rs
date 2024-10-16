use crate::actions::sync::sync_all_game_files;
use crate::download_manager::{DownloadConfig, DownloadManager};
use crate::runtime::{get_client, get_config};
use tracing::{info, warn};

pub async fn download(game: &Option<String>) {
    match game {
        None => download_all().await,
        Some(game) => download_game(game).await,
    };
}

pub async fn download_game(game: &str) {
    info!("Start Syncing: {game}");
    if !get_config().get_csv_path_for_game(game).as_path().is_file() {
        sync_all_game_files(game).await;
    }
    download_all_files(game).await;
}

async fn download_all() {
    let games = get_client().get_games_list().await.unwrap_or_default();
    for game in games.iter() {
        info!("Start Syncing: {game}");
        download_all_files(game).await;
    }
}

pub async fn download_all_files(game: &str) {
    let file_list_option = get_config().get_game_file_list(game);

    match file_list_option {
        None => warn!("Game \"{game}\" not found."),
        Some(file_list) => {
            let mut download_configs = Vec::with_capacity(file_list.len());
            for file in file_list {
                let dc = DownloadConfig::new(
                    file.generate_download_link(get_config().server_url.as_str()),
                    file.file_path.clone(),
                );
                download_configs.push(dc);
            }

            let mut download_manager = DownloadManager::with(download_configs);

            download_manager
                .download_all_to(get_config().client_games_folder.as_path())
                .await;
        }
    }
}
