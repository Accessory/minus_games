use crate::runtime::{get_client, send_event, MinusGamesClientEvents};
use chrono::DateTime;
use minus_games_utils::set_file_modified_time;
use reqwest::Response;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use tracing::{trace, warn};

pub struct DownloadConfig {
    pub url: String,
    pub to: String,
    pub to_final: Option<PathBuf>,
}

impl DownloadConfig {
    pub fn new(url: String, to: String) -> Self {
        Self {
            url,
            to,
            to_final: None,
        }
    }
}

pub struct DownloadManager {
    download_list: Vec<DownloadConfig>,
}

impl DownloadManager {
    pub fn with(download_list: Vec<DownloadConfig>) -> Self {
        Self { download_list }
    }
    pub async fn download_all_to(&mut self, path: &Path) {
        let processes: usize = std::thread::available_parallelism().unwrap().get();
        let semaphore = Arc::new(Semaphore::new(processes));
        let mut joinings: Vec<JoinHandle<()>> = Vec::new();
        send_event(MinusGamesClientEvents::StartDownloadingFiles(
            self.download_list.len(),
        ))
        .await;
        for mut config in self.download_list.drain(0..) {
            let pass = semaphore.clone().acquire_owned().await.unwrap();
            config.to_final = Some(path.join(config.to.as_str()));
            // println!("{}", config.to_final.as_ref().unwrap().display());
            let handle = tokio::spawn(async move {
                trace!(
                    "Currently Running Downloads: {}",
                    processes - pass.semaphore().available_permits()
                );
                send_event(MinusGamesClientEvents::StartDownloadingFile).await;
                download_to(config).await;
                send_event(MinusGamesClientEvents::FinishedDownloadingFile).await;
                drop(pass);
            });
            joinings.push(handle);
        }

        for join_handle in joinings.drain(0..) {
            join_handle.await.unwrap();
        }
        send_event(MinusGamesClientEvents::FinishedDownloadingFiles).await;
    }
}

pub async fn download_to(download_config: DownloadConfig) {
    let to = download_config.to_final.unwrap();
    if to.exists() {
        return;
    }

    let response = get_client().get(&download_config.url).await;

    if !response.status().is_success() {
        warn!(
            "Download {} failed with: {} - {}",
            download_config.url,
            response.status(),
            response.text().await.unwrap()
        );
        return;
    }

    download_loop(response, to.as_path()).await;
}

pub async fn download_loop(mut response: Response, to: &Path) {
    trace!("Download From: {} - To: {}", response.url(), to.display());

    tokio::fs::create_dir_all(to.parent().unwrap())
        .await
        .unwrap();

    let mut download_file = match File::create(&to).await {
        Ok(file) => file,
        Err(err) => {
            warn!("File could not be created {err} - File: {}", to.display());
            return;
        }
    };

    loop {
        let read_result = response.chunk().await.unwrap();

        if let Some(bytes) = read_result {
            if let Err(err) = download_file.write_all(&bytes).await {
                warn!("Download failed with: {err}");
            }
        } else {
            break;
        };
    }

    if let Some(last_modified_header_value) = response.headers().get("last-modified") {
        if let Ok(last_modified_str) = last_modified_header_value.to_str() {
            if let Ok(last_modified) = DateTime::parse_from_rfc2822(last_modified_str) {
                set_file_modified_time(to, last_modified.into());
            }
        }
    }
}
