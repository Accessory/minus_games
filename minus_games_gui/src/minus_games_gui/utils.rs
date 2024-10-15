// use iced::widget::image;
// use std::path::Path;

// pub(crate) async fn fetch_image(image_path: &str) -> Option<image::Handle> {
//     if image_path.starts_with("http") {
//         let bytes = reqwest::get(image_path).await.ok()?.bytes().await.ok()?;
//         Some(image::Handle::from_bytes(bytes))
//     } else if Path::new(image_path).is_file() {
//         Some(image::Handle::from_path(image_path))
//     } else {
//         None
//     }
// }

// pub(crate) fn fetch_image_sync(image_path: &str) -> Option<image::Handle> {
//     if image_path.starts_with("http") {
//         let bytes = reqwest::blocking::get(image_path).ok()?.bytes().ok()?;
//         Some(image::Handle::from_bytes(bytes))
//     } else if Path::new(image_path).is_file() {
//         Some(image::Handle::from_path(image_path))
//     } else {
//         None
//     }
// }
