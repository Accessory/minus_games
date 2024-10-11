use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) enum ModalCallback {
    DeleteGame(String),
    RepairGame(String),
    OpenGameFolder(PathBuf),
}
