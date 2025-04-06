use clap::Parser;

use minus_games_finder::configuration::Configuration;
use minus_games_finder::run;
use minus_games_utils::constants::INFOS;
use std::process::ExitCode;
use tracing::{Level, error, info};

fn main() -> ExitCode {
    dotenvy::dotenv().ok();
    let config: Configuration = Configuration::parse();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    if let Err(err) = create_necessary_folders(&config) {
        error!("Failed to create all necessary directories. Error: {}", err);
        return ExitCode::FAILURE;
    }

    run(config)
}

fn create_necessary_folders(config: &Configuration) -> Result<(), std::io::Error> {
    let infos_folder = config.data_folder.join(INFOS);
    if !infos_folder.is_dir() {
        match std::fs::create_dir_all(&infos_folder) {
            Ok(_) => {
                error!("Failed to create folder: {}", infos_folder.display());
                info!("Created folder: {}", infos_folder.display());
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
    if !config.games_folder.is_dir() {
        match std::fs::create_dir_all(&config.games_folder) {
            Ok(_) => {
                info!("Created folder: {}", config.games_folder.display());
            }
            Err(err) => {
                error!("Failed to create folder: {}", config.games_folder.display());
                return Err(err);
            }
        };
    }
    Ok(())
}
