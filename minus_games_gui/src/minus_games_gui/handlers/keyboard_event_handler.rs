use crate::minus_games_gui::messages::minus_games_gui_message::MinusGamesGuiMessage;
use crate::minus_games_gui::{FILTER_ID, MinusGamesGui};
use crate::runtime::{CLOSING, IS_IN_FOCUS, SCROLLABLE_ID};
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::{Key, key};
use iced::mouse::Button;
use iced::widget::scrollable::{RelativeOffset, snap_to};
use iced::widget::text_input;
use iced::{Event, Task, mouse, widget, window};
use std::sync::atomic::Ordering::Relaxed;
use tracing::{info, trace};

pub(crate) fn handle_system_events(
    minus_games_gui: &mut MinusGamesGui,
    event: Event,
) -> Task<MinusGamesGuiMessage> {
    match event {
        Event::Keyboard(KeyPressed {
            key: Key::Character(character),
            modifiers,
            ..
        }) => {
            if character.as_str() == "f" && modifiers.control() {
                return Task::batch([
                    snap_to(SCROLLABLE_ID.clone(), RelativeOffset::START),
                    text_input::focus(FILTER_ID),
                ]);
            }
        }
        Event::Keyboard(KeyPressed {
            key: Key::Named(named),
            modifiers,
            ..
        }) => match named {
            key::Named::Tab => {
                return if modifiers.shift() {
                    widget::focus_previous()
                } else {
                    widget::focus_next()
                };
            }
            key::Named::ArrowUp => {
                return if modifiers.control() {
                    Task::done(MinusGamesGuiMessage::CurrentPositionUp(3))
                } else {
                    Task::done(MinusGamesGuiMessage::CurrentPositionUp(1))
                };
            }
            key::Named::ArrowDown => {
                return if modifiers.control() {
                    Task::done(MinusGamesGuiMessage::CurrentPositionDown(3))
                } else {
                    Task::done(MinusGamesGuiMessage::CurrentPositionDown(1))
                };
            }
            key::Named::Enter => {
                return Task::done(MinusGamesGuiMessage::StartCurrentPosition);
            }
            key::Named::Home => {
                return Task::done(MinusGamesGuiMessage::CurrentPositionUp(usize::MAX));
            }
            key::Named::End => {
                return Task::done(MinusGamesGuiMessage::CurrentPositionDown(usize::MAX));
            }
            key::Named::PageUp => {
                return Task::done(MinusGamesGuiMessage::CurrentPositionUp(3));
            }
            key::Named::PageDown => {
                return Task::done(MinusGamesGuiMessage::CurrentPositionDown(3));
            }
            _ => {}
        },
        Event::Mouse(mouse::Event::ButtonPressed(Button::Back)) => {
            return Task::done(MinusGamesGuiMessage::BackFromSettings(false));
        }
        Event::Mouse(mouse::Event::CursorMoved { .. }) => {
            minus_games_gui.last_input_mouse = true;
        }
        Event::Window(window::Event::CloseRequested) => {
            info!("Close Application!");
            return Task::perform(
                MinusGamesGui::close(),
                MinusGamesGuiMessage::CloseApplication,
            );
        }
        Event::Window(window::Event::Resized(size)) => {
            minus_games_gui.size = size;
        }
        Event::Window(window::Event::Unfocused) => {
            trace!("Lost Focus");
            IS_IN_FOCUS.store(false, Relaxed);
        }
        Event::Window(window::Event::Focused) => {
            trace!("Is in Focus");
            IS_IN_FOCUS.store(true, Relaxed);
        }
        // Event::Window(window::Event::Opened { position: _, size }) => {
        //     minus_games_gui.size = size;
        // }
        Event::Window(window::Event::Closed) => {
            CLOSING.store(true, Relaxed);
        }
        _ => {}
    }

    Task::none()
}
