use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::runtime::{CLOSING, IS_IN_FOCUS};
use gilrs::{Button, EventType, GamepadId, Gilrs, GilrsBuilder};
use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, Stream};
use iced::stream;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use tokio::task::yield_now;
use tracing::{debug, trace};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum AxisEvent {
    Neural,
    Down,
    Up,
}

impl From<f32> for AxisEvent {
    fn from(value: f32) -> Self {
        if value > 0.7 {
            AxisEvent::Up
        } else if value < -0.7 {
            AxisEvent::Down
        } else {
            AxisEvent::Neural
        }
    }
}

pub(crate) fn gamepad_input_handler() -> impl Stream<Item = MinusGamesGuiMessage> {
    stream::channel(
        64,
        |mut output: mpsc::Sender<MinusGamesGuiMessage>| async move {
            let mut gilrs = GilrsBuilder::new().set_update_state(true).build().unwrap();
            let mut last_message_send = None;
            let mut last_button_pressed = None;
            let mut last_id_used = None;
            let mut block_input_until = tokio::time::Instant::now();
            let mut last_axes_code = None;
            let mut last_axes_event = None;
            loop {
                if !IS_IN_FOCUS.load(Relaxed) {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    continue;
                }

                if let Some(gilrs::Event { event, id, .. }) = gilrs.next_event() {
                    // trace!("Start Controller Event");
                    if CLOSING.load(Relaxed) {
                        return;
                    }

                    let mut axis_event_option = None;

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
                        EventType::AxisChanged(gilrs::Axis::LeftStickY, delta, code) => {
                            last_axes_code = Some(code);
                            axis_event_option = Some(delta.into());
                        }
                        _ => continue,
                    }

                    if tokio::time::Instant::now() < block_input_until {
                        output.flush().await.ok();
                        yield_now().await;
                        // tokio::time::sleep_until(block_input_until).await;
                        continue;
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
                                last_axes_event = None;
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
                                last_axes_event = None;
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
                                last_button_pressed = Some(button);
                                last_axes_event = None;
                                last_id_used = Some(id);
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(500))
                                    .unwrap();
                            }
                            Button::East => {
                                debug!("Pressed East");
                                output.send(MinusGamesGuiMessage::BackAction).await.ok();
                                last_button_pressed = Some(button);
                                last_axes_event = None;
                                last_id_used = Some(id);
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(500))
                                    .unwrap();
                            }
                            Button::Start => {
                                debug!("Pressed Start");
                                output.send(MinusGamesGuiMessage::StartAction).await.ok();
                                last_button_pressed = Some(button);
                                last_axes_event = None;
                                last_id_used = Some(id);
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(500))
                                    .unwrap();
                            }
                            Button::Select => {
                                debug!("Pressed Select");
                                output.send(MinusGamesGuiMessage::ReloadAction).await.ok();
                                last_button_pressed = Some(button);
                                last_axes_event = None;
                                last_id_used = Some(id);
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(500))
                                    .unwrap();
                            }
                            _ => {
                                trace!("Button: {button:?}")
                            }
                        }
                        continue;
                    } else if let Some(axis_event) = axis_event_option {
                        match axis_event {
                            AxisEvent::Neural => {
                                last_message_send = None;
                                last_button_pressed = None;
                                last_id_used = None;
                                last_axes_code = None;
                                last_axes_event = None;
                                continue;
                            }
                            AxisEvent::Down => {
                                let to_move: usize = get_move_distance(&gilrs, id);
                                debug!("Axis Down");
                                output
                                    .send(MinusGamesGuiMessage::CurrentPositionDown(to_move))
                                    .await
                                    .ok();
                                last_message_send =
                                    Some(MinusGamesGuiMessage::CurrentPositionDown(to_move));
                                last_button_pressed = None;
                                last_axes_event = Some(AxisEvent::Down);
                                last_id_used = Some(id);
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(250))
                                    .unwrap();
                            }
                            AxisEvent::Up => {
                                let to_move: usize = get_move_distance(&gilrs, id);
                                debug!("Axis Up");
                                output
                                    .send(MinusGamesGuiMessage::CurrentPositionUp(to_move))
                                    .await
                                    .ok();
                                last_message_send =
                                    Some(MinusGamesGuiMessage::CurrentPositionUp(to_move));
                                last_button_pressed = None;
                                last_axes_event = Some(AxisEvent::Up);
                                last_id_used = Some(id);
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(250))
                                    .unwrap();
                            }
                        }
                    }
                } else if CLOSING.load(Relaxed) {
                    return;
                } else if last_button_pressed.is_some() {
                    // info!("Last Button Event");
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
                } else if let Some(last_event) = last_axes_event {
                    // trace!("Last Axis Event");
                    if tokio::time::Instant::now() < block_input_until {
                        output.flush().await.ok();
                        yield_now().await;
                        continue;
                    }

                    let last_id = match last_id_used.as_ref() {
                        Some(value) => *value,
                        None => continue,
                    };

                    let last_message = last_message_send.clone().unwrap();

                    if let Some(gamepad) = gilrs.connected_gamepad(last_id) {
                        if let Some(code) = last_axes_code {
                            let new_axes_event: AxisEvent = match gamepad.state().axis_data(code) {
                                Some(value) => value.value().into(),
                                None => continue,
                            };

                            if new_axes_event == last_event {
                                output.send(last_message).await.ok();
                                output.flush().await.ok();
                                block_input_until = tokio::time::Instant::now()
                                    .checked_add(Duration::from_millis(100))
                                    .unwrap();
                            }

                            continue;
                        }
                    } else {
                        last_message_send = None;
                        last_button_pressed = None;
                        last_id_used = None;
                        // tokio::time::sleep(Duration::from_millis(100)).await;
                        continue;
                    }
                } else {
                    // info!("Do nothing!");
                    tokio::time::sleep(Duration::from_millis(200)).await;
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
