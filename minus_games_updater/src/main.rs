use crate::configuration::{Configuration, OS};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use chrono::{DateTime, Utc};
use clap::Parser;
use filetime::FileTime;
use futures_util::stream::StreamExt;
use minus_games_models::sync_file_info::SyncFileInfo;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, Url};
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use tokio::io::AsyncWriteExt;
use tracing::level_filters::LevelFilter;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

mod configuration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Init
    dotenvy::dotenv().ok();
    let filter = EnvFilter::default()
        .add_directive(LevelFilter::INFO.into())
        .add_directive("minus_games_updater=debug".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        // .with_max_level(Level::INFO)
        .init();

    let config: Configuration = Configuration::parse();

    info!("Config:\n{config}");

    // Init Client
    let client = if config.username.is_some() && config.password.is_some() {
        let mut headers = HeaderMap::new();
        let encoded_part = BASE64_STANDARD.encode(format!(
            "{}:{}",
            config.username.as_ref().unwrap(),
            config.password.as_ref().unwrap()
        ));
        headers.append(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", encoded_part)).unwrap(),
        );
        reqwest::ClientBuilder::new()
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .unwrap()
    } else {
        reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap()
    };

    let url = match Url::parse(config.server_url.as_str()) {
        Ok(url) => url,
        Err(err) => {
            warn!("Url could not be parsed. Error: {}", err.to_string());
            return;
        }
    };

    update_client(&config, &client, &url).await;
    update_gui(&config, &client, &url).await;

    info!("Download complete!");
}

async fn update_gui(config: &Configuration, client: &Client, url: &Url) {
    // Update Gui Client
    let url_path = match config.for_os {
        OS::Windows => "gui/windows/info",
        OS::Linux => "gui/linux/info",
    };

    let gui_url = url.join(url_path).unwrap();
    let response = match client.get(gui_url).send().await {
        Ok(response) => response,
        Err(err) => {
            warn!("Failed to get infos. Error: {}", err.to_string());
            return;
        }
    };

    if !response.status().is_success() {
        warn!(
            "Update response is not successful. Code {} - Message: {}",
            response.status().as_str(),
            response.text().await.unwrap()
        );
        return;
    }

    let sync_file_info: SyncFileInfo = response.json().await.unwrap();

    let to = config.get_gui_to();

    if to.exists() {
        let metadata = to.metadata().unwrap();
        let modified_system_time = metadata.modified().unwrap();
        let modified_date_time = DateTime::<Utc>::from(modified_system_time);
        let size = metadata.len();

        if modified_date_time.timestamp() == sync_file_info.last_modified.timestamp()
            && size == sync_file_info.size
        {
            info!("Gui is up to date. Nothing to do.");
            return;
        }
    }

    let client_url_part = match config.for_os {
        OS::Windows => "/download/minus_games_gui.exe",
        OS::Linux => "/download/minus_games_gui",
    };

    info!("Download Gui");
    let download_url = Url::parse(&config.server_url)
        .unwrap()
        .join(client_url_part)
        .unwrap();
    let response = client.get(download_url).send().await.unwrap();

    if !response.status().is_success() {
        warn!(
            "Failed to download Gui with: {} - {}",
            response.status(),
            response.text().await.unwrap()
        );
        return;
    }

    let mut stream = response.bytes_stream();
    {
        let mut file = tokio::fs::File::create(to.as_path()).await.unwrap();
        while let Some(chunk) = stream.next().await {
            file.write_all(&chunk.unwrap()).await.unwrap();
        }
        file.flush().await.unwrap();
    }

    let last_modified = FileTime::from_unix_time(sync_file_info.last_modified.timestamp(), 0);
    match filetime::set_file_times(to.as_path(), last_modified, last_modified) {
        Ok(_) => {}
        Err(_) => {
            warn!("Failed to set filetime");
        }
    }

    #[cfg(target_family = "unix")]
    if config.for_os == OS::Linux {
        let mode = to.metadata().unwrap().permissions().mode() | 0o111;
        std::fs::set_permissions(to.as_path(), std::fs::Permissions::from_mode(mode)).unwrap();
    }
}

async fn update_client(config: &Configuration, client: &Client, url: &Url) {
    // Update Cli Client
    let url_path = match config.for_os {
        OS::Windows => "client/windows/info",
        OS::Linux => "client/linux/info",
    };

    let client_url = url.join(url_path).unwrap();
    let response = match client.get(client_url).send().await {
        Ok(response) => response,
        Err(err) => {
            warn!("Failed to get client infos. Error: {}", err.to_string());
            return;
        }
    };

    if !response.status().is_success() {
        warn!(
            "Update response is not successful. Code {} - Message: {}",
            response.status().as_str(),
            response.text().await.unwrap()
        );
        return;
    }

    let sync_file_info: SyncFileInfo = response.json().await.unwrap();

    let to = config.get_client_to();

    if to.exists() {
        let metadata = to.metadata().unwrap();
        let modified_system_time = metadata.modified().unwrap();
        let modified_date_time = DateTime::<Utc>::from(modified_system_time);
        let size = metadata.len();

        if modified_date_time.timestamp() == sync_file_info.last_modified.timestamp()
            && size == sync_file_info.size
        {
            info!("Client is up to date. Nothing to do.");
            return;
        }
    }

    let client_url_part = match config.for_os {
        OS::Windows => "/download/minus_games_client.exe",
        OS::Linux => "/download/minus_games_client",
    };

    info!("Download Client");
    let download_url = Url::parse(&config.server_url)
        .unwrap()
        .join(client_url_part)
        .unwrap();
    let response = client.get(download_url).send().await.unwrap();

    if !response.status().is_success() {
        warn!(
            "Failed to download Client with: {} - {}",
            response.status(),
            response.text().await.unwrap()
        );
        return;
    }

    let mut stream = response.bytes_stream();
    {
        let mut file = match tokio::fs::File::create(to.as_path()).await {
            Ok(file) => file,
            Err(err) => {
                warn!("Failed to create '{}' with Error: {}", to.display(), err);
                return;
            }
        };
        while let Some(chunk) = stream.next().await {
            file.write_all(&chunk.unwrap()).await.unwrap();
        }
        file.flush().await.unwrap();
    }

    let last_modified = FileTime::from_unix_time(sync_file_info.last_modified.timestamp(), 0);
    match filetime::set_file_times(to.as_path(), last_modified, last_modified) {
        Ok(_) => {}
        Err(_) => {
            warn!("Failed to set filetime");
        }
    }

    #[cfg(target_family = "unix")]
    if config.for_os == OS::Linux {
        let mode = to.metadata().unwrap().permissions().mode() | 0o111;
        std::fs::set_permissions(to.as_path(), std::fs::Permissions::from_mode(mode)).unwrap();
    }
    info!("Download complete!");
}
