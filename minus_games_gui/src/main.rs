#![windows_subsystem = "windows"]
use crate::minus_games_gui::MinusGamesGui;
use crate::minus_games_gui::configuration::Mode;
use crate::minus_games_gui::configuration::complete_configuration::CompleteConfiguration;
use crate::runtime::{GUI_CONFIG, get_gui_config};
use clap::Parser;
use iced::window::icon::from_rgba;
use iced::{Font, Settings, application};
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
    let complete_configuration_parsing_result = CompleteConfiguration::try_parse();

    unsafe {
        #[allow(static_mut_refs)]
        match complete_configuration_parsing_result {
            Ok(complete_configuration) => {
                let (gui_configuration, client_configuration) =
                    complete_configuration.into_gui_configuration_and_client_configuration();
                CONFIG.get_or_insert(client_configuration);
                GUI_CONFIG.get_or_insert(gui_configuration);
            }
            #[cfg(not(target_family = "windows"))]
            Err(err) => {
                eprintln!("Failed to parse Arguments:\n {err}");
                return ExitCode::FAILURE;
            }
            #[cfg(target_family = "windows")]
            Err(err) => {
                use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};
                use windows::core::PCWSTR;

                fn to_wide(s: &str) -> Vec<u16> {
                    s.encode_utf16().chain(Some(0)).collect()
                }

                let message_w = to_wide(&err.to_string());
                let title_w = to_wide("Failed to parse Arguments");

                MessageBoxW(
                    None,
                    PCWSTR(message_w.as_ptr()),
                    PCWSTR(title_w.as_ptr()),
                    MB_OK | MB_ICONERROR,
                );
                return ExitCode::FAILURE;
            }
        }
    }

    println!("{}", get_config());
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
        EnvFilter::default()
            .add_directive(LevelFilter::DEBUG.into())
            .add_directive("wgpu_core=WARN".parse().unwrap())
            .add_directive("wgpu_hal=WARN".parse().unwrap())
            .add_directive("iced_futures=WARN".parse().unwrap())
            .add_directive("cosmic_text=WARN".parse().unwrap())
    } else {
        EnvFilter::default()
            .add_directive(LevelFilter::INFO.into())
            .add_directive("iced=WARN".parse().unwrap())
            .add_directive("wgpu_core=WARN".parse().unwrap())
            .add_directive("wgpu_hal=WARN".parse().unwrap())
            .add_directive("iced_futures=WARN".parse().unwrap())
            .add_directive("cosmic_text=WARN".parse().unwrap())
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    if let Err(err) = get_config().create_necessary_folders() {
        error!("Failed to create all necessary directories. Error: {}", err);
        return ExitCode::FAILURE;
    }

    match get_gui_config().mode {
        Mode::Gui => {
            static ICON: &[u8] = include_bytes!("../../other/assets/common/MinusGamesV2.png");
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
            .scale_factor(|minus_games_gui| {
                minus_games_gui
                    .scale
                    .or_else(|| get_gui_config().scale)
                    .unwrap_or(1.0)
            })
            .theme(MinusGamesGui::get_theme)
            .exit_on_close_request(false)
            .font(include_bytes!(
                "./minus_games_gui/assets/fonts/MonaspiceArNerdFont-Regular.otf"
            ))
            .default_font(Font::new(&get_gui_config().font))
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
