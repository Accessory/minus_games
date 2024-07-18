use crate::configuration::Configuration;
use crate::minus_games_client::MinusGamesClient;
use clap::Parser;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Configuration> = LazyLock::new(Configuration::parse);

pub static CLIENT: LazyLock<MinusGamesClient> = LazyLock::new(|| {
    MinusGamesClient::new(
        CONFIG.server_url.as_str(),
        CONFIG.username.as_ref(),
        CONFIG.password.as_ref(),
    )
});
