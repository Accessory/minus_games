use crate::runtime::CONFIG;
use tracing::info;

pub fn scan_for_games() {
    let config = minus_games_finder::configuration::Configuration {
        games_folder: CONFIG.client_games_folder.clone(),
        data_folder: CONFIG.client_folder.clone(),
        cache_folder: None,
        cleanup_data_folder: false,
        keep_existing_configs: true,
        filter: None,
    };

    info!("Run Finder");
    minus_games_finder::run(config);
    info!("Finder finished");
}
