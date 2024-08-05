use crate::configuration::Configuration;
use crate::minus_games_client::MinusGamesClient;
use clap::Parser;
use std::sync::atomic::AtomicBool;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Configuration> = LazyLock::new(Configuration::parse);

pub static CLIENT: LazyLock<MinusGamesClient> = LazyLock::new(|| {
    MinusGamesClient::new(
        CONFIG.server_url.as_str(),
        CONFIG.username.as_ref(),
        CONFIG.password.as_ref(),
    )
});

pub static OFFLINE: AtomicBool = AtomicBool::new(false);

#[macro_export]
macro_rules! offline_to_none {
    () => {
        if OFFLINE.load(Relaxed) {
            debug!("Client is offline!");
            return None;
        }
    };
}

#[macro_export]
macro_rules! offline_to_return {
    () => {
        if OFFLINE.load(Relaxed) {
            debug!("Client is offline!");
            return;
        }
    };
}
