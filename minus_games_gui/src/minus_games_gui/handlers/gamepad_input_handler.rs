use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::runtime::{CLOSING, IS_IN_FOCUS};
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
        64,
        |mut output: mpsc::Sender<MinusGamesGuiMessage>| async move {
            let mut gilrs = GilrsBuilder::new().set_update_state(true).build().unwrap();
            let mut last_message_send = None;
            let mut last_button_pressed = None;
            let mut last_id_used = None;
            let mut block_input_until = tokio::time::Instant::now();
            loop {
                if !IS_IN_FOCUS.load(Relaxed) {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    continue;
                }

                if let Some(gilrs::Event { event, id, .. }) = gilrs.next_event() {
                    if CLOSING.load(Relaxed) {
                        return;
                    }

                    match event {
                        EventType::ButtonPressed(_, _) => {}
                        EventType::ButtonReleased(_, _) => {
                            debug!("Button Released");
                            block_input_until = tokio::time::Instant::now();
                            last_message_send = None;
                            last_button_pressed = None;
                            last_id_used = None;
                            continue;
                        }
                        _ => continue,
                    }

                    if tokio::time::Instant::now() < block_input_until {
                        output.flush().await.ok();
                        yield_now().await;
                        tokio::time::sleep_until(block_input_until).await;
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
                                last_message_send =
                                    Some(MinusGamesGuiMessage::CurrentPositionDown(to_move));
                                last_button_pressed = Some(button);
                                last_id_used = Some(id);
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(250))
                                    .unwrap();
                            }
                            Button::DPadUp => {
                                let to_move: usize = get_move_distance(&gilrs, id);
                                debug!("Pressed Up");
                                output
                                    .send(MinusGamesGuiMessage::CurrentPositionUp(to_move))
                                    .await
                                    .ok();
                                last_message_send =
                                    Some(MinusGamesGuiMessage::CurrentPositionUp(to_move));
                                last_button_pressed = Some(button);
                                last_id_used = Some(id);
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(250))
                                    .unwrap();
                            }
                            Button::South => {
                                debug!("Pressed South");
                                output
                                    .send(MinusGamesGuiMessage::StartCurrentPosition)
                                    .await
                                    .ok();
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(250))
                                    .unwrap();
                            }
                            Button::East => {
                                debug!("Pressed East");
                                output.send(MinusGamesGuiMessage::BackAction).await.ok();
                            }
                            Button::Start => {
                                debug!("Pressed Start");
                                output.send(MinusGamesGuiMessage::StartAction).await.ok();
                            }
                            Button::Select => {
                                debug!("Pressed Select");
                                output.send(MinusGamesGuiMessage::ReloadAction).await.ok();
                            }
                            _ => {
                                println!("Button: {button:?}")
                            }
                        }
                        continue;
                    }
                } else if CLOSING.load(Relaxed) {
                    return;
                } else if last_button_pressed.is_some() {
                    if tokio::time::Instant::now() < block_input_until {
                        output.flush().await.ok();
                        yield_now().await;
                        continue;
                    }

                    let last_button = *last_button_pressed.as_ref().unwrap();
                    let last_id = *last_id_used.as_ref().unwrap();
                    let last_message = last_message_send.clone().unwrap();

                    if let Some(gamepad) = gilrs.connected_gamepad(last_id) {
                        if gamepad.is_pressed(last_button) {
                            output.send(last_message).await.ok();
                            output.flush().await.ok();
                            block_input_until = tokio::time::Instant::now()
                                .checked_add(Duration::from_millis(100))
                                .unwrap();
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
    if let Some(gamepad) = gilrs.connected_gamepad(id)
        && (gamepad.is_pressed(Button::LeftTrigger) || gamepad.is_pressed(Button::RightTrigger))
    {
        3
    } else {
        1
    }
}
