use crate::runtime::get_config;
use minus_games_models::game_file_info::GameFileInfo;
use std::io::BufReader;
use tracing::warn;

pub fn delete_game(game: &str, purge: bool) {
    let csv = get_config().get_csv_path_for_game(game);
    if purge {
        let game_path = get_config().get_game_path(game);
        match std::fs::remove_dir_all(game_path) {
            Ok(_) => {}
            Err(err) => {
                warn!("Game not found: {}", err);
            }
        }
    } else {
        let csv_file = std::fs::File::open(csv.as_path()).unwrap();
        let csv_buf_reader = BufReader::new(csv_file);
        let mut reader = csv::ReaderBuilder::new().from_reader(csv_buf_reader);
        let files: Vec<GameFileInfo> = reader.deserialize().map(|i| i.unwrap()).collect();
        for to_delete in files {
            let to_remove = get_config().client_games_folder.join(to_delete.file_path);
            match std::fs::remove_file(to_remove) {
                Ok(_) => {}
                Err(err) => {
                    warn!("Game not found: {}", err);
                }
            };
        }
    }

    match std::fs::remove_file(csv) {
        Ok(_) => {}
        Err(err) => {
            warn!("CSV not found: {}", err);
        }
    };
    let json = get_config().get_game_infos_path_from_game(game);
    match std::fs::remove_file(json) {
        Ok(_) => {}
        Err(err) => {
            warn!("Json not found: {}", err);
        }
    }

    std::fs::remove_dir_all(get_config().get_game_additions_path(game)).ok();

    get_config().unmark_last_time_played(game);
    get_config().unmark_games_as_dirty(game);
}

pub fn delete_game_info_files(game: &str) {
    let json_path = get_config().get_game_infos_path_from_game(game);
    if json_path.is_file() {
        std::fs::remove_file(json_path).unwrap();
    }
    let csv_path = get_config().get_csv_path_for_game(game);
    if csv_path.is_file() {
        std::fs::remove_file(csv_path).unwrap();
    }
}
