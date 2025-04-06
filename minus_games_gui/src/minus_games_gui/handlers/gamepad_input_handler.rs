use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::runtime::CLOSING;
use gilrs::{Button, EventType, GamepadId, Gilrs, GilrsBuilder};
use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, Stream};
use iced::stream;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use tokio::task::yield_now;
use tracing::debug;

pub(crate) fn gamepad_input_handler() -> impl Stream<Item = MinusGamesGuiMessage> {
    stream::channel(
        512,
        |mut output: mpsc::Sender<MinusGamesGuiMessage>| async move {
            let mut gilrs = GilrsBuilder::new().set_update_state(true).build().unwrap();
            let mut last_message_send = None;
            let mut last_button_pressed = None;
            let mut last_id_used = None;
            loop {
                if let Some(gilrs::Event { event, id, .. }) =
                    gilrs.next_event_blocking(Some(Duration::from_millis(300)))
                {
                    if CLOSING.load(Relaxed) {
                        return;
                    }

                    match event {
                        EventType::ButtonPressed(_, _) => {}
                        EventType::ButtonReleased(_, _) => {
                            last_message_send = None;
                            last_button_pressed = None;
                            last_id_used = None;
                            continue;
                        }
                        _ => continue,
                    }

                    if let EventType::ButtonPressed(button, _) = event {
                        match button {
                            Button::DPadDown => {
                                let to_move: usize = get_move_distance(&gilrs, id);
                                debug!("Pressed Down");
                                output
                                    .send(MinusGamesGuiMessage::CurrentPositionDown(to_move))
                                    .await
                                    .ok();
                                output.flush().await.ok();

                                last_message_send =
                                    Some(MinusGamesGuiMessage::CurrentPositionDown(to_move));
                                last_button_pressed = Some(button);
                                last_id_used = Some(id);
                                yield_now().await;
                                tokio::time::sleep(Duration::from_millis(200)).await;
                            }
                            Button::DPadUp => {
                                let to_move: usize = get_move_distance(&gilrs, id);
                                debug!("Pressed Up");
                                output
                                    .send(MinusGamesGuiMessage::CurrentPositionUp(to_move))
                                    .await
                                    .ok();
                                output.flush().await.ok();

                                last_message_send =
                                    Some(MinusGamesGuiMessage::CurrentPositionUp(to_move));
                                last_button_pressed = Some(button);
                                last_id_used = Some(id);
                                yield_now().await;
                                tokio::time::sleep(Duration::from_millis(200)).await;
                            }
                            Button::South => {
                                debug!("Pressed South");
                                output
                                    .send(MinusGamesGuiMessage::StartCurrentPosition)
                                    .await
                                    .ok();
                                output.flush().await.ok();
                                yield_now().await;
                                tokio::time::sleep(Duration::from_millis(100)).await;
                            }
                            Button::East => {
                                debug!("Pressed East");
                                output.send(MinusGamesGuiMessage::BackAction).await.ok();
                                output.flush().await.ok();
                                yield_now().await;
                                tokio::time::sleep(Duration::from_millis(100)).await;
                            }
                            Button::Start => {
                                debug!("Pressed Start");
                                output.send(MinusGamesGuiMessage::StartAction).await.ok();
                                output.flush().await.ok();
                                yield_now().await;
                                tokio::time::sleep(Duration::from_millis(100)).await;
                            }
                            Button::Select => {
                                debug!("Pressed Select");
                                output.send(MinusGamesGuiMessage::ReloadAction).await.ok();
                                output.flush().await.ok();
                                yield_now().await;
                                tokio::time::sleep(Duration::from_millis(100)).await;
                            }
                            _ => {
                                // println!("Button: {:?}", button)
                            }
                        }
                    }
                } else if CLOSING.load(Relaxed) {
                    return;
                } else if last_button_pressed.is_some() {
                    let last_button = *last_button_pressed.as_ref().unwrap();
                    let last_id = *last_id_used.as_ref().unwrap();
                    let last_message = last_message_send.clone().unwrap();

                    if let Some(gamepad) = gilrs.connected_gamepad(last_id) {
                        if gamepad.is_pressed(last_button) {
                            output.send(last_message).await.ok();
                            output.flush().await.ok();
                            tokio::time::sleep(Duration::from_millis(100)).await;
                        } else {
                            last_message_send = None;
                            last_button_pressed = None;
                            last_id_used = None;
                            continue;
                        }
                    } else {
                        last_message_send = None;
                        last_button_pressed = None;
                        last_id_used = None;
                        continue;
                    }
                }
            }
        },
    )
}

fn get_move_distance(gilrs: &Gilrs, id: GamepadId) -> usize {
    if let Some(gamepad) = gilrs.connected_gamepad(id) {
        if gamepad.is_pressed(Button::LeftTrigger) || gamepad.is_pressed(Button::RightTrigger) {
            return 3;
        }
    }
    1
}
