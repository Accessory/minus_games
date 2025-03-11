use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, Stream, StreamExt};
use iced::stream;
use minus_games_client::runtime::{OFFLINE, get_client, get_config};
use std::sync::atomic::Ordering::Relaxed;
use tokio::task::yield_now;

pub(crate) fn lazy_image_download_handler() -> impl Stream<Item = MinusGamesGuiMessage> {
    stream::channel(
        512,
        |mut output: mpsc::Sender<MinusGamesGuiMessage>| async move {
            let (sender, mut receiver) = mpsc::channel(512);
            output
                .send(MinusGamesGuiMessage::LazyImageDownloaderReady(sender))
                .await
                .ok();
            while let Some((game, installed, card_id)) = receiver.next().await {
                if OFFLINE.load(Relaxed) {
                    continue;
                }

                let download_to = if installed {
                    get_config().get_game_additions_header_path(&game)
                } else {
                    get_config().get_game_additions_header_tmp_folder(&game)
                };

                get_client()
                    .download_game_additions_header_file(&game, &download_to)
                    .await;
                if download_to.is_file() {
                    output
                        .send(MinusGamesGuiMessage::LazyImageUpdateCard(
                            card_id,
                            iced::widget::image::Handle::from_path(&download_to),
                        ))
                        .await
                        .ok();
                }
                yield_now().await;
            }
        },
    )
}
