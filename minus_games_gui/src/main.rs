#![windows_subsystem = "windows"]
use crate::minus_games_gui::MinusGamesGui;
use iced::application;
use minus_games_client::runtime::{get_config, OFFLINE};
use std::sync::atomic::Ordering::Relaxed;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod minus_games_gui;

fn main() -> iced::Result {
    if let Some(config_dir) = dirs::config_local_dir() {
        let config_path = config_dir.join("minus_games_gui").join("config");
        if config_path.exists() {
            dotenvy::from_filename_override(config_path).ok();
        }
    }
    dotenvy::dotenv().ok();

    println!("Config:");
    println!("{}", get_config());
    OFFLINE.store(get_config().offline, Relaxed);

    // Logging
    let filter = if get_config().verbose {
        EnvFilter::default().add_directive(LevelFilter::DEBUG.into())
    } else {
        EnvFilter::default().add_directive(LevelFilter::INFO.into())
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    application("Minus Games", MinusGamesGui::update, MinusGamesGui::view)
        .subscription(MinusGamesGui::batch_subscription)
        .exit_on_close_request(false)
        .run_with(MinusGamesGui::init)
}
