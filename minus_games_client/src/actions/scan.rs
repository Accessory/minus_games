use crate::runtime::get_config;
use log::warn;
use std::process::ExitCode;
use tracing::info;

pub fn scan_for_games() {
    let config = minus_games_finder::configuration::Configuration {
        games_folder: get_config().client_games_folder.clone(),
        data_folder: get_config().client_folder.clone(),
        cache_folder: get_config().client_cache_folder.clone(),
        cleanup_data_folder: false,
        keep_existing_configs: true,
        filter: None,
    };

    info!("Run Finder");
    let status_code_rtn = minus_games_finder::run(config);
    if status_code_rtn == ExitCode::SUCCESS {
        info!("Finder finished");
    } else {
        warn!("Finder finished with errors")
    }
}
