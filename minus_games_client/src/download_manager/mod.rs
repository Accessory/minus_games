use crate::runtime::{MinusGamesClientEvents, STOP_DOWNLOAD, get_client, send_event};
use chrono::DateTime;
use minus_games_utils::set_file_modified_time;
use reqwest::Response;
use std::io::Write;
use std::num::NonZero;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use tokio::spawn;
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
        let parallelism = std::thread::available_parallelism()
            .unwrap_or(NonZero::new(1).unwrap())
            .get();
        let processes: usize = (parallelism / 2).max(1);
        let semaphore = Arc::new(Semaphore::new(processes));
        let mut joinings: Vec<JoinHandle<()>> = Vec::new();
        send_event(MinusGamesClientEvents::StartDownloadingFiles(
            self.download_list.len(),
        ))
        .await;
        for mut config in self.download_list.drain(0..) {
            if STOP_DOWNLOAD.load(Relaxed) {
                break;
            }
            let pass = semaphore.clone().acquire_owned().await.unwrap();
            config.to_final = Some(path.join(config.to.as_str()));
            let handle = spawn(async move {
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
            if let Err(err) = join_handle.await {
                warn!("Error downloading files: {}", err);
            }
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

    let parent = to.parent().unwrap();
    match std::fs::create_dir_all(to.parent().unwrap()) {
        Ok(_) => {}
        Err(err) => {
            warn!(
                "Failed to create download directory - {} - Err: {}",
                parent.display(),
                err
            );
        }
    }

    let download_file = match std::fs::File::create(to) {
        Ok(file) => file,
        Err(err) => {
            warn!("File could not be created {err} - File: {}", to.display());
            return;
        }
    };

    let mut writer = std::io::BufWriter::with_capacity(128 * 1024, download_file);

    // let header = response.headers().get("last-modified").cloned();

    loop {
        if STOP_DOWNLOAD.load(Relaxed) {
            break;
            // return;
        }

        if let Some(bytes) = response.chunk().await.unwrap() {
            if let Err(err) = writer.write(&bytes) {
                warn!("Download failed with: {err}");
            }
        } else {
            break;
        };
    }

    // let mut stream = response
    //     .bytes_stream()
    //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
    // let mut reader = tokio_util::io::StreamReader::new(stream);
    // tokio::io::copy_buf(&mut reader, &mut writer).await.unwrap();

    // download_file.flush().ok();
    writer.flush().ok();
    std::mem::drop(writer);

    // if let Some(last_modified_header_value) = header
    if let Some(last_modified_header_value) = response.headers().get("last-modified")
        && let Ok(last_modified_str) = last_modified_header_value.to_str()
        && let Ok(last_modified) = DateTime::parse_from_rfc2822(last_modified_str)
    {
        set_file_modified_time(to, last_modified.into());
    }
}
