use crate::runtime::CLIENT;
use tracing::info;

pub async fn list_json() {
    let games = CLIENT.get_games_list().await;
    print!("{}", serde_json::to_string_pretty(&games).unwrap());
}

pub async fn list() {
    let games = CLIENT.get_games_list().await.unwrap_or_default();
    info!("List Games:");
    for game in games {
        info!("{game}");
    }
}
