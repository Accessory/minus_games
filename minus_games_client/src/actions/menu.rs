use super::run::{run_game, run_game_synced};
use crate::actions::delete::delete_game;
use crate::actions::download::download_game;
use crate::actions::repair::repair_game;
use crate::actions::scan::scan_for_games;
#[cfg(target_family = "unix")]
use crate::actions::sync::sync_all_game_files;
use crate::actions::sync::{download_sync_for_game, sync_game_infos, upload_sync_for_game};
use crate::runtime::{get_all_games, get_config, get_installed_games, send_event, CLIENT};
#[cfg(target_family = "unix")]
use crate::utils::make_executable_from_path;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use tracing::info;

#[cfg(target_family = "unix")]
pub async fn select_game_to_play() {
    let games = get_all_games().await;

    if games.is_empty() {
        info!("No games found!");
        return;
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Game:")
        .default(0)
        .items(games.as_slice())
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        let game = games.get(selection).expect("Selection out of range");

        info!("Sync game files.");
        sync_all_game_files(game).await;
        info!("Download Saves.");
        download_sync_for_game(game).await;

        let mut to = dirs::cache_dir().unwrap().join("minus_games_client");
        std::fs::create_dir_all(&to).unwrap();
        to.push("run_in_term.sh");
        info!("Write run file to: {}", to.display());
        std::fs::write(
            &to,
            format!(
                "#!/bin/sh\nexec {} run-game-synced \"{game}\"",
                std::env::current_exe().unwrap().to_str().unwrap()
            ),
        )
        .unwrap();
        make_executable_from_path(&to);
    } else {
        info!("Nothing selected!");
    }
}

pub async fn start_menu() {
    const MENU_ITEMS: [&str; 9] = [
        "Sync & Start",
        "Start",
        "Download",
        "Delete",
        "Repair",
        "Upload Saves",
        "Download Saves",
        "Scan for Games",
        "Quit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a action:")
        .default(0)
        .items(MENU_ITEMS.as_slice())
        .interact_opt()
        .unwrap();

    match selection {
        Some(0) => select_game_to_run_synced().await,
        Some(1) => select_game().await,
        Some(2) => select_game_to_download().await,
        Some(3) => select_game_to_delete(true),
        Some(4) => select_repair().await,
        Some(5) => select_upload_saves().await,
        Some(6) => select_download_saves().await,
        Some(7) => scan_for_games(),
        Some(8) => {
            info!("Quitting now.")
        }
        _ => info!("Nothing selected - Quitting now."),
    }
}

pub async fn select_upload_saves() {
    let installed_games = get_installed_games();

    if installed_games.is_empty() {
        info!("No games installed");
        return;
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Game to upload saves:")
        .default(0)
        .items(installed_games.as_slice())
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        let game = installed_games.get(selection).unwrap();
        upload_sync_for_game(game).await;
    } else {
        info!("Nothing selected!")
    }
}
pub async fn select_download_saves() {
    let installed_games = get_installed_games();

    if installed_games.is_empty() {
        info!("No games installed");
        return;
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Game to download saves:")
        .default(0)
        .items(installed_games.as_slice())
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        let game = installed_games.get(selection).unwrap();
        download_sync_for_game(game).await;
    } else {
        info!("Nothing selected!")
    }
}
pub async fn select_repair() {
    let installed_games = get_installed_games();

    if installed_games.is_empty() {
        info!("No games installed");
        return;
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Game to repair:")
        .default(0)
        .items(installed_games.as_slice())
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        let game = installed_games.get(selection).unwrap();
        repair_game(game).await;
    } else {
        info!("Nothing selected!")
    }
}

pub fn select_game_to_delete(purge: bool) {
    let installed_games = get_installed_games();

    if installed_games.is_empty() {
        info!("No games installed");
        return;
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Game to delete:")
        .default(0)
        .items(installed_games.as_slice())
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        let game = installed_games.get(selection).unwrap();
        delete_game(game, purge);
    } else {
        info!("Nothing selected!")
    }
}

pub async fn select_game_to_download() {
    let mut games = CLIENT.get_games_list().await.unwrap_or_default();
    if games.is_empty() {
        info!("No games found!");
        return;
    }

    let installed_games = get_installed_games();

    games.retain(|game| !installed_games.contains(game));

    if games.is_empty() {
        info!("No game awailable for downloading.");
        return;
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Game to download:")
        .default(0)
        .items(games.as_slice())
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        let game = games.get(selection).unwrap();
        if !get_config()
            .get_json_path_from_game(game)
            .as_path()
            .is_file()
        {
            sync_game_infos(game).await;
        }
        if !get_config().get_game_path(game).is_dir() {
            download_game(game).await;
            send_event(format!("Game \"{game}\" downloaded successfully!").into()).await;
        }
    } else {
        info!("Nothing selected!");
    }
}

pub async fn select_game_to_run_synced() {
    let games = get_all_games().await;
    if games.is_empty() {
        info!("No games found!");
        return;
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Game to sync and play:")
        .default(0)
        .items(games.as_slice())
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        let game = games.get(selection).expect("Selection out of range");
        run_game_synced(game).await;
    } else {
        info!("Nothing selected!");
    }
}

pub async fn select_game() {
    let mut installed_games = get_installed_games();
    let games = CLIENT.get_games_list().await.unwrap_or_default();

    for game in games {
        if !installed_games.contains(&game) {
            installed_games.push(game);
        }
    }

    if installed_games.is_empty() {
        info!("No games found!");
        return;
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Game to play:")
        .default(0)
        .items(installed_games.as_slice())
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        let game = installed_games.get(selection).unwrap();
        if !get_config()
            .get_json_path_from_game(game)
            .as_path()
            .is_file()
        {
            sync_game_infos(game).await;
        }
        if !get_config().get_game_path(game).is_dir() {
            download_game(game).await;
        }
        run_game(game).await;
    } else {
        info!("Nothing selected!");
    }
}

pub async fn select_download() {
    println!("Select Game:");
    let games = CLIENT.get_games_list().await.unwrap_or_default();
    for (idx, game) in games.iter().enumerate() {
        println!("{idx:<2} - {game}");
    }
    println!();
    println!("Input number: ");
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);

    let idx: usize = line.trim().parse().unwrap();
    download_game(games.get(idx).unwrap()).await;
}
