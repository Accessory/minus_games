#![feature(let_chains)]

use clap::Parser;

use minus_games_finder::configuration::Configuration;
use minus_games_finder::run;
use std::process::ExitCode;
use tracing::Level;

fn main() -> ExitCode {
    dotenvy::dotenv().ok();
    let config: Configuration = Configuration::parse();

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    run(config)
}
