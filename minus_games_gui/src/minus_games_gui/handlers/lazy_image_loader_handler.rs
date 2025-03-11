// use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
// use iced::futures::channel::mpsc;
// use iced::futures::{SinkExt, Stream, StreamExt};
// use iced::stream;
// use std::time::Duration;
// use iced::futures::channel::mpsc::TrySendError;
// use tokio::time::sleep;
// 
// pub(crate) fn lazy_image_loader_handler() -> impl Stream<Item = MinusGamesGuiMessage> {
//     stream::channel(512, |mut output| async move {
//         let (sender, mut receiver) = mpsc::channel(512);
//         output
//             .send(MinusGamesGuiMessage::LazyImageLoadingReady(sender))
//             .await
//             .ok();
// 
//         while let Some((path, card_id)) = receiver.next().await {
//             if path.is_file() {
//                 let handle = iced::widget::image::Handle::from_path(path);
// 
//                 loop {
//                     match output.try_send(MinusGamesGuiMessage::LazyImageLoadingUpdateCard(
//                         card_id, handle.clone(),
//                     )) {
//                         Ok(_) => {
//                             break;
//                         }
//                         Err(err) => {
//                             match err { TrySendError { .. } => {
//                                 sleep(Duration::from_millis(1000 / 60)).await;
//                             } }
//                         },
//                     }
//                 }
//             }
//         }
//     })
// }
