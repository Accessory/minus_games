use crate::runtime::get_client;
use tracing::info;

pub async fn list_json() {
    let games = get_client().get_games_list().await;
    print!("{}", serde_json::to_string_pretty(&games).unwrap());
}

pub async fn list() {
    let games = get_client().get_games_list().await.unwrap_or_default();
    info!("List Games:");
    for game in games {
        info!("{game}");
    }
}
