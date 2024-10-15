use crate::download_manager::download_loop;
use crate::offline_to_none;
use crate::runtime::{get_config, OFFLINE};
use crate::utils::{encode_problem_chars, get_csv_name, get_json_name};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::{DateTime, Utc};
use log::{debug, warn};
use minus_games_models::sync_file_info::SyncFileInfo;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, IF_MODIFIED_SINCE};
use reqwest::{multipart, Body, Client, Response, StatusCode, Url};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering::Relaxed;
use std::time::SystemTime;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct MinusGamesClient {
    client: Client,
    url: Url,
}

impl MinusGamesClient {
    pub async fn get(&self, url: &str) -> Response {
        self.client.get(url).send().await.unwrap()
    }

    pub async fn upload_sync_file(
        &self,
        game: &str,
        folder_hash: &str,
        sync_file_info: SyncFileInfo,
        upload_file_path: PathBuf,
    ) {
        debug!("Uploading: {}", upload_file_path.display());
        let url = self
            .url
            .join("/sync/")
            .unwrap()
            .join(&format!("{game}/{folder_hash}"))
            .unwrap();

        let to_upload = match tokio::fs::File::open(upload_file_path).await {
            Ok(to_upload) => to_upload,
            Err(err) => {
                warn!("Failed Uploading Sync Files with: {err}");
                return;
            }
        };
        let stream = FramedRead::new(to_upload, BytesCodec::new());
        let file_body = Body::wrap_stream(stream);
        let file_stream = multipart::Part::stream(file_body)
            .file_name(sync_file_info.file_name.clone())
            .mime_str(mime::APPLICATION_OCTET_STREAM.as_ref())
            .unwrap();

        let form = multipart::Form::new()
            .text("file_name", sync_file_info.file_name)
            .text("file_path", sync_file_info.file_path)
            .text("size", sync_file_info.size.to_string())
            .text("last_modified", sync_file_info.last_modified.to_rfc3339())
            .part("upload_data", file_stream);

        let response = match self.client.post(url).multipart(form).send().await {
            Ok(response) => response,
            Err(_) => {
                return;
            }
        };

        if !response.status().is_success() {
            warn!(
                "Failed to upload sync file: {} - {}",
                response.status(),
                response.text().await.unwrap()
            );
        }
    }

    pub async fn get_sync_file_list(
        &self,
        game: &str,
        folder_hash: &str,
    ) -> Option<Vec<SyncFileInfo>> {
        let url = self
            .url
            .join("/sync/")
            .unwrap()
            .join(&format!("{game}/{folder_hash}"))
            .unwrap();
        debug!("URL: {}", url);
        let result = self.client.get(url).send().await.ok()?;

        if !result.status().is_success() {
            warn!(
                "Failed to get sync file list: {} - {}",
                result.status(),
                result.text().await.unwrap()
            );
            return None;
        }

        result.json().await.ok()?
    }

    pub async fn download_game_infos_if_modified(&self, game: &str) -> bool {
        let json_name = get_json_name(game);
        let from = self
            .url
            .join("/games/data/")
            .unwrap()
            .join(&encode_problem_chars(json_name.as_str()))
            .unwrap();
        let to = get_config().get_json_path(json_name.as_str());
        self.download_file_if_modified(from, to.as_path()).await
    }

    pub async fn download_game_files_if_modified(&self, game: &str) -> bool {
        let csv_name = get_csv_name(game);
        let from = self
            .url
            .join("/games/data/")
            .unwrap()
            .join(&encode_problem_chars(csv_name.as_str()))
            .unwrap();
        let to = get_config().get_csv_path(csv_name.as_str());
        self.download_file_if_modified(from, to.as_path()).await
    }

    // pub async fn download_file_list(&self, game: &str, ) {
    //     let csv_name = get_csv_name(game);
    //     let from = self
    //         .url
    //         .join("/games/data/")
    //         .unwrap()
    //         .join(&encode_questinmark(csv_name.as_str()))
    //         .unwrap();
    //     let to = get_config().get_csv_path(csv_name.as_str());
    //     self.download_file(from, to.as_path()).await;
    // }

    pub async fn download_infos(&self, game: &str) {
        let json_name = get_json_name(game);
        let from = self
            .url
            .join("/games/data/")
            .unwrap()
            .join(&encode_problem_chars(json_name.as_str()))
            .unwrap();
        let to = get_config().get_json_path(json_name.as_str());
        let handle_info = self.download_file_if_not_exists(from, to);
        let csv_name = get_csv_name(game);
        let from = self
            .url
            .join("/games/data/")
            .unwrap()
            .join(&encode_problem_chars(csv_name.as_str()))
            .unwrap();
        let to = get_config().get_csv_path(csv_name.as_str());
        let handle_files = self.download_file_if_not_exists(from, to);
        tokio::join!(handle_info, handle_files);
    }

    pub async fn download_sync_file(
        &self,
        game: &str,
        folder_hash: &str,
        file_path: &str,
        to: &Path,
    ) {
        let url = self
            .url
            .join("/sync/")
            .unwrap()
            .join(&format!("{}/{folder_hash}/", encode_problem_chars(game)))
            .unwrap()
            .join(file_path)
            .unwrap();
        self.download_file(url, to).await;
    }

    pub async fn download_file_if_modified(&self, from: Url, to: &Path) -> bool {
        let modified: Option<SystemTime> = match to.metadata() {
            Ok(metadata) => metadata.modified().ok(),
            Err(_) => None,
        };

        let response = match modified {
            Some(modified) => match self
                .client
                .get(from.as_str())
                .header(
                    IF_MODIFIED_SINCE,
                    <DateTime<Utc>>::from(modified)
                        .format("%a, %d %b %Y %H:%M:%S GMT")
                        .to_string(),
                )
                .send()
                .await
            {
                Ok(response) => response,
                Err(_) => {
                    return false;
                }
            },
            None => match self.client.get(from.as_str()).send().await {
                Ok(response) => response,
                Err(_) => {
                    warn!("Failed to get from url: {from}");
                    return false;
                }
            },
        };

        let status = response.status();
        if status == StatusCode::NOT_MODIFIED {
            debug!("File not modified: {from}");
            return false;
        }

        if status == StatusCode::NOT_FOUND {
            warn!("File not found: {from}");
            return false;
        }

        if !status.is_success() {
            warn!(
                "Failed to download file: {from} with status: {}",
                status.as_str()
            );
            return false;
        }

        download_loop(response, to).await;
        true
    }

    pub async fn download_file(&self, from: Url, to: &Path) {
        let response = self.client.get(from.clone()).send().await.unwrap();

        let status = response.status();

        if !status.is_success() {
            warn!(
                "Failed to download file: {from} with status: {}",
                status.as_str()
            );
            return;
        }

        download_loop(response, to).await
    }

    pub async fn download_file_if_not_exists(&self, from: Url, to: PathBuf) {
        if to.is_file() {
            return;
        }

        self.download_file(from, to.as_path()).await;
    }

    pub fn new(url: &str, username: Option<&String>, password: Option<&String>) -> Self {
        let client = if username.is_some() && password.is_some() {
            let mut headers = HeaderMap::new();
            let encoded_part =
                BASE64_STANDARD.encode(format!("{}:{}", username.unwrap(), password.unwrap()));
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
        Self {
            client,
            url: Url::parse(url).unwrap(),
        }
    }

    pub async fn get_games_list(&self) -> Option<Vec<String>> {
        offline_to_none!();
        let url = self.url.join("/games/list").unwrap();
        let result = match self.client.get(url).send().await {
            Ok(response) => response,
            Err(_) => {
                OFFLINE.store(true, Relaxed);
                return None;
            }
        };

        if !result.status().is_success() {
            warn!(
                "Failed to get games list: {} - {}",
                result.status(),
                result.text().await.unwrap()
            );
            return None;
        }

        result.json().await.ok()?
    }

    // pub async fn get_game_file_list(&self, game: &str) -> Option<Vec<GameFileInfo>> {
    //     let csv_name = format!("{game}.csv");
    //     let url = self
    //         .url
    //         .join("/games/data/")
    //         .unwrap()
    //         .join(&csv_name)
    //         .unwrap();
    //     let result = self
    //         .client
    //         .get(url)
    //         .send()
    //         .await
    //         .expect("Failed to query user data");

    //     if result.status() == StatusCode::NOT_FOUND {
    //         return None;
    //     }

    //     let bytes = result.bytes().await.unwrap();
    //     let mut reader = csv::ReaderBuilder::new().from_reader(&*bytes);
    //     let rtn = reader.deserialize().map(|i| i.unwrap()).collect();

    //     rtn
    // }
}
