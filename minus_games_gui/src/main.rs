#![windows_subsystem = "windows"]

use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::configuration::Mode;
use crate::runtime::get_gui_config;
use clap::Parser;
use iced::window::icon::from_rgba;
use iced::{Font, Settings, application};
use minus_games_client::configuration::Configuration;
use minus_games_client::run_cli;
use minus_games_client::runtime::{CONFIG, OFFLINE, SYNC, SYNC_TESTED, get_config};
use std::process::ExitCode;
use std::sync::atomic::Ordering::Relaxed;
use tracing::error;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

mod minus_games_gui;
mod runtime;

fn main() -> ExitCode {
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

                if ["--theme", "--mode", "--font"].contains(&item.as_str()) {
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
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_BUILD_DATE")
    );
    println!(
        "Build Source Date: {} - Git Hash: {}",
        env!("VERGEN_GIT_COMMIT_DATE"),
        env!("VERGEN_GIT_SHA")
    );

    OFFLINE.store(get_config().offline, Relaxed);
    SYNC.store(get_config().sync, Relaxed);
    if !get_config().sync {
        SYNC_TESTED.store(true, Relaxed);
    }

    // Logging
    let filter = if get_config().verbose {
        EnvFilter::default().add_directive(LevelFilter::DEBUG.into())
    } else {
        EnvFilter::default()
            .add_directive(LevelFilter::INFO.into())
            .add_directive("iced=WARN".parse().unwrap())
            .add_directive("wgpu_core=WARN".parse().unwrap())
            .add_directive("wgpu_hal=WARN".parse().unwrap())
            .add_directive("iced_futures=WARN".parse().unwrap())
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    if let Err(err) = get_config().create_necessary_folders() {
        error!("Failed to create all necessary directories. Error: {}", err);
        return ExitCode::FAILURE;
    }

    match get_gui_config().mode {
        Mode::Gui => {
            static ICON: &[u8] = include_bytes!("../../other/assets/common/MinusGames.jpg");
            let image = image::load_from_memory(ICON).unwrap();
            let window_settings = iced::window::Settings {
                icon: Some(
                    from_rgba(image.into_rgba8().to_vec(), 128, 128).expect("Failed to load icon"),
                ),
                // size: iced::Size::new(1280.0, 800.0),
                // max_size: Some(iced::Size::new(1280.0, 800.0)),
                ..Default::default()
            };

            let settings = Settings {
                id: Some("minus_games_gui".to_string()),
                ..Default::default()
            };

            let result = application(
                MinusGamesGui::init,
                MinusGamesGui::update,
                MinusGamesGui::view,
            )
            .title(MinusGamesGui::title)
            .settings(settings)
            .subscription(MinusGamesGui::batch_subscription)
            .window(window_settings)
            .scale_factor(|minus_games_gui| minus_games_gui.scale.unwrap_or(1.0))
            .theme(MinusGamesGui::get_theme)
            .exit_on_close_request(false)
            .font(include_bytes!(
                "./minus_games_gui/assets/fonts/MonaspiceArNerdFont-Regular.otf"
            ))
            .default_font(Font::with_name(&get_gui_config().font))
            .run();

            if let Err(err) = result {
                error!("Failed to create a {}", err);
                return ExitCode::FAILURE;
            }
        }
        Mode::Cli => {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Could not create a tokio runtime")
                .block_on(async { run_cli().await });
        }
    }

    ExitCode::SUCCESS
}
