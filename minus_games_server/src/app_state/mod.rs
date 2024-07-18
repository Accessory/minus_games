use crate::auth::session_manager::SessionManager;
use crate::auth::user_handler::UserHandler;
use crate::configuration::Configuration;
use axum::extract::multipart::Field;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

pub struct AppState {
    pub config: Configuration,
    pub user_handler: Arc<UserHandler>,
    pub session_manager: Arc<RwLock<SessionManager>>,
    pub clear_sessions: Arc<RwLock<Option<JoinHandle<()>>>>,
}

impl AppState {
    pub fn does_game_exist(&self, game: &str) -> bool {
        let json_name = format!("{game}.json");
        self.config.data_folder.join(json_name).is_file()
    }

    pub async fn write_save_file(&self, game: &str, mut field: Field<'_>) {
        let save_folder_path: PathBuf = self.get_save_folder(game);
        tokio::fs::create_dir_all(save_folder_path.as_path())
            .await
            .unwrap();
        let file_name = field.file_name().unwrap();
        let save_file_path = save_folder_path.join(file_name);
        let mut save_file = File::create(save_file_path).await.unwrap();
        loop {
            let chunk = field.chunk().await.unwrap();

            match chunk {
                None => break,
                Some(bytes) => {
                    let _ = save_file.write(&bytes).await.unwrap();
                }
            }
        }
    }

    fn get_save_folder(&self, game: &str) -> PathBuf {
        const SAVES: &str = "saves";
        self.config.data_folder.join(SAVES).join(game)
    }
}
