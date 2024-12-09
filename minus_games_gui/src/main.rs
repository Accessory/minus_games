#![windows_subsystem = "windows"]

use crate::minus_games_gui::configuration::Mode;
use crate::minus_games_gui::MinusGamesGui;
use crate::runtime::get_gui_config;
use clap::Parser;
use iced::application;
use iced::window::icon::from_rgba;
use minus_games_client::configuration::Configuration;
use minus_games_client::run_cli;
use minus_games_client::runtime::{get_config, CONFIG, OFFLINE};
use std::sync::atomic::Ordering::Relaxed;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod minus_games_gui;
mod runtime;

fn main() -> iced::Result {
    if let Some(config_dir) = dirs::config_local_dir() {
        let config_path = config_dir.join("minus_games_gui").join("config");
        if config_path.exists() {
            dotenvy::from_filename_override(config_path).ok();
        }
    }
    dotenvy::dotenv_override().ok();

    println!("Config:");
    println!("{}", unsafe {
        #[allow(static_mut_refs)]
        CONFIG.get_or_insert_with(|| {
            let mut parse_list: Vec<String> = Vec::new();
            let mut is_not_ok = false;
            for (i, item) in std::env::args().enumerate() {
                if i == 0 {
                    parse_list.push(item);
                    continue;
                }

                if is_not_ok {
                    is_not_ok = false;
                    continue;
                }

                if ["--theme", "--mode"].contains(&item.as_str()) {
                    is_not_ok = true;
                    continue;
                }
                if ["--fullscreen"].contains(&item.as_str()) {
                    continue;
                }
                parse_list.push(item);
            }

            Configuration::parse_from(parse_list)
        })
    });
    println!("{}", get_gui_config());
    println!(
        "Version: {} Build on: {}",
        env!("VERGEN_GIT_COMMIT_DATE"),
        env!("VERGEN_GIT_SHA")
    );
    println!(
        "Build Source Date: {} - Git Hash: {}",
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_BUILD_DATE")
    );

    OFFLINE.store(get_config().offline, Relaxed);

    // Logging
    let filter = if get_config().verbose {
        EnvFilter::default().add_directive(LevelFilter::DEBUG.into())
    } else {
        EnvFilter::default().add_directive(LevelFilter::INFO.into())
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    match get_gui_config().mode {
        Mode::Gui => {
            static ICON: &[u8] = include_bytes!("../../other/assets/common/MinusGames.jpg");
            let image = image::load_from_memory(ICON).unwrap();
            let window_settings = iced::window::Settings {
                icon: Some(
                    from_rgba(image.into_rgba8().to_vec(), 128, 128).expect("Failed to load icon"),
                ),
                ..Default::default()
            };

            application("Minus Games", MinusGamesGui::update, MinusGamesGui::view)
                .subscription(MinusGamesGui::batch_subscription)
                .window(window_settings)
                .theme(MinusGamesGui::get_theme)
                .run_with(MinusGamesGui::init)
        }
        Mode::Cli => {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not create a tokio runtime")
                .block_on(async { run_cli().await });
            Ok(())
        }
    }
}
