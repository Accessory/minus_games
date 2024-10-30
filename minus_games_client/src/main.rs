use minus_games_client::configuration::ClientActions::ListJson;
use minus_games_client::run_cli;
use minus_games_client::runtime::get_config;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Configuration
    if let Some(config_dir) = dirs::config_local_dir() {
        let config_path = config_dir.join("minus_games_client").join("config");
        if config_path.exists() {
            dotenvy::from_filename_override(config_path).ok();
        }
    }
    dotenvy::dotenv_override().ok();

    // Logging
    let filter = if get_config().verbose {
        EnvFilter::default()
            .add_directive(LevelFilter::TRACE.into())
            .add_directive("minus_games_client=debug".parse().unwrap())
    } else {
        EnvFilter::default().add_directive(LevelFilter::INFO.into())
        // .add_directive("minus_games_client=debug".parse().unwrap())
    };
    tracing_subscriber::fmt().with_env_filter(filter).init();

    if !get_config().action.as_ref().is_some_and(|a| a == &ListJson) {
        println!("Minus Games Version {}", env!("CARGO_PKG_VERSION"));
        println!("Config:");
        println!("{}", get_config());
    }

    run_cli().await;
}
