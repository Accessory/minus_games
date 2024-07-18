use crate::actions::download::download_all_files;
use crate::actions::sync::sync_game_infos;
use crate::runtime::CONFIG;
use tracing::warn;

pub async fn repair_game(game: &str) {
    sync_game_infos(game).await;
    if let Some(game_file_infos) = CONFIG.get_game_info(game) {
        for info in game_file_infos {
            let file_path = CONFIG.client_games_folder.join(info.file_path);
            if let Ok(metadata) = file_path.as_path().metadata() {
                if metadata.len() != info.size && file_path.as_path().is_file() {
                    std::fs::remove_file(file_path).unwrap();
                }
            }
        }
    } else {
        return warn!("No game infos found for {game}");
    }

    download_all_files(game).await;
}
